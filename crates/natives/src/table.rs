use better_any::{Tid, TidAble};
use initia_move_storage::table_resolver::TableResolver;
use initia_move_types::iterator::Order;
use initia_move_types::table::{TableChange, TableChangeSet, TableHandle, TableInfo};
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::gas_algebra::NumArgs;
use move_core_types::identifier::Identifier;
use move_core_types::{
    account_address::AccountAddress, effects::Op, gas_algebra::NumBytes, value::MoveTypeLayout,
    vm_status::StatusCode,
};
use move_vm_runtime::native_functions::{NativeContext, NativeFunctionTable};
use move_vm_types::values::{Struct, Vector};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{GlobalValue, Reference, StructRef, Value},
};
use sha3::{Digest, Sha3_256};
use smallvec::{smallvec, SmallVec};

use std::ops::{Bound, RangeBounds};
use std::{
    cell::RefCell,
    collections::{btree_map::Entry, BTreeMap, BTreeSet, VecDeque},
};

use crate::interface::{
    RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
};
use crate::safely_pop_arg;

/// UID prefix is used to generate unique address from the txn hash.
const UID_PREFIX: [u8; 4] = [0, 0, 0, 2];

/// A table operation, for supporting cost calculation.
pub enum TableOperation {
    NewHandle,
    Destroy,
    Insert,
    Borrow,
    Length,
    Remove,
    Contains,
}

/// The native table context extension. This needs to be attached to the NativeContextExtensions
/// value which is passed into session functions, so its accessible from natives of this
/// extension.
#[derive(Tid)]
pub struct NativeTableContext<'a> {
    resolver: &'a mut dyn TableResolver,
    session_id: [u8; 32],
    table_data: RefCell<TableData>,
    iterators: RefCell<Vec<TableIter>>,
}

// See stdlib/error.move
const _ECATEGORY_INVALID_STATE: u64 = 0x3;
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

const ALREADY_EXISTS: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;
const NOT_FOUND: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 101;

// Move side raises this
const _NOT_EMPTY: u64 = (_ECATEGORY_INVALID_STATE << 16) + 102;

// ===========================================================================================
// Private Data Structures and Constants

/// A structure representing mutable data of the NativeTableContext. This is in a RefCell
/// of the overall context so we can mutate while still accessing the overall context.
#[derive(Default)]
struct TableData {
    new_tables: BTreeMap<TableHandle, TableInfo>,
    removed_tables: BTreeSet<TableHandle>,
    tables: BTreeMap<TableHandle, Table>,
}

/// A structure representing a single table.
struct Table {
    handle: TableHandle,
    key_layout: MoveTypeLayout,
    value_layout: MoveTypeLayout,
    content: BTreeMap<Vec<u8>, GlobalValue>,
}

/// A structure representing a table iterator
struct TableIter {
    iterator_id: u32,
    handle: TableHandle,
    /// The changes is built from BTreeMap range function,
    /// so the contents are sorted
    changes: BTreeSet<Vec<u8>>,
    /// next item loaded from the iterator
    next: Option<(Value, Value)>,
    order: Order,
}

/// The field index of the `handle` field in the `Table` Move struct.
const HANDLE_FIELD_INDEX: usize = 0;

/// The field index of the `iterator` field in the `TableIter` Move struct.
const ITERATOR_ID_FIELD_INDEX: usize = 0;

// =========================================================================================
// Implementation of Native Table Context

impl<'a> NativeTableContext<'a> {
    /// Create a new instance of a native table context. This must be passed in via an
    /// extension into VM session functions.
    pub fn new(session_id: [u8; 32], resolver: &'a mut dyn TableResolver) -> Self {
        Self {
            resolver,
            session_id,
            table_data: Default::default(),
            iterators: Default::default(),
        }
    }

    /// Computes the change set from a NativeTableContext.
    pub fn into_change_set(self) -> PartialVMResult<TableChangeSet> {
        let NativeTableContext { table_data, .. } = self;
        let TableData {
            new_tables,
            removed_tables,
            tables,
        } = table_data.into_inner();
        let mut changes = BTreeMap::new();
        for (handle, table) in tables {
            let Table {
                value_layout,
                content,
                ..
            } = table;
            let mut entries = BTreeMap::new();
            for (key, gv) in content {
                let op = match gv.into_effect() {
                    Some(op) => op,
                    None => continue,
                };

                match op {
                    Op::New(val) => {
                        let bytes = serialize(&value_layout, &val)?;
                        entries.insert(key, Op::New(bytes));
                    }
                    Op::Modify(val) => {
                        let bytes = serialize(&value_layout, &val)?;
                        entries.insert(key, Op::Modify(bytes));
                    }
                    Op::Delete => {
                        entries.insert(key, Op::Delete);
                    }
                }
            }
            if !entries.is_empty() {
                changes.insert(
                    handle,
                    TableChange {
                        value_layout,
                        entries,
                    },
                );
            }
        }
        Ok(TableChangeSet {
            new_tables,
            removed_tables,
            changes,
        })
    }

    pub fn resolve_table_entry(
        &self,
        handle: &TableHandle,
        key: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let table_data = self.table_data.borrow();
        match table_data.tables.get(handle){
            Some(table) => {
                match table.content.get(key){
                    Some(gv) => {
                        match gv.borrow_global() {
                            Ok(val) => {
                                return Ok(Some(serialize(&table.value_layout, &val)?));
                            }
                            Err(e) => {
                                if gv.is_mutated() {
                                    // handle the case where the value is deleted
                                    return Err(e.into());
                                }
                            },
                        }
                    },
                    None => (),
                };
            },
            None => (),
        };

        self.resolver.resolve_table_entry(handle, key)
    }
}

impl TableData {
    /// Gets or creates a new table in the TableData. This initializes information about
    /// the table, like the type layout for keys and values.
    fn get_or_create_table(
        &mut self,
        context: &NativeContext,
        handle: TableHandle,
        key_ty: &Type,
        value_ty: &Type,
    ) -> PartialVMResult<&mut Table> {
        Ok(match self.tables.entry(handle) {
            Entry::Vacant(e) => {
                let key_layout = get_type_layout(context, key_ty)?;
                let value_layout = get_type_layout(context, value_ty)?;
                let table = Table {
                    handle,
                    key_layout,
                    value_layout,
                    content: Default::default(),
                };
                e.insert(table)
            }
            Entry::Occupied(e) => e.into_mut(),
        })
    }
}

impl Table {
    fn get_or_create_global_value(
        &mut self,
        context: &NativeTableContext,
        key: Vec<u8>,
    ) -> PartialVMResult<(&mut GlobalValue, Option<Option<NumBytes>>)> {
        Ok(match self.content.entry(key) {
            Entry::Vacant(entry) => {
                let (gv, loaded) = match context
                    .resolver
                    .resolve_table_entry(&self.handle, entry.key())
                    .map_err(|err| {
                        partial_extension_error(format!("remote table resolver failure: {}", err))
                    })? {
                    Some(val_bytes) => {
                        let val = deserialize(&self.value_layout, &val_bytes)?;
                        (
                            GlobalValue::cached(val)?,
                            Some(NumBytes::new(val_bytes.len() as u64)),
                        )
                    }
                    None => (GlobalValue::none(), None),
                };
                (entry.insert(gv), Some(loaded))
            }
            Entry::Occupied(entry) => (entry.into_mut(), None),
        })
    }
}

impl TableIter {
    #[allow(clippy::type_complexity)]
    fn load_next_key(
        &mut self,
        resolver: &mut dyn TableResolver,
    ) -> PartialVMResult<(Option<Vec<u8>>, Option<Option<NumBytes>>)> {
        let res = resolver.next_key(self.iterator_id).map_err(|err| {
            partial_extension_error(format!("remote table resolver failure: {}", err))
        })?;

        let (next_item, loaded) = match res {
            Some(key_bytes) => {
                let num_bytes = key_bytes.len() as u64;

                (Some(key_bytes), Some(Some(NumBytes::new(num_bytes))))
            }
            None => (None, Some(None)),
        };

        if let Some(next_item) = next_item {
            self.changes.insert(next_item);
        }

        let next_key_bytes = match self.order {
            Order::Ascending => self.changes.iter().next().map(|k| k.to_vec()),
            Order::Descending => self.changes.iter().next_back().map(|k| k.to_vec()),
        };

        Ok((
            next_key_bytes.map(|k| self.changes.take(&k).unwrap()),
            loaded,
        ))
    }
}

// =========================================================================================
// Native Function Implementations

/// Returns all natives for tables.
pub fn all_natives(
    table_addr: AccountAddress,
    builder: &mut SafeNativeBuilder,
) -> NativeFunctionTable {
    builder
        .make_named_natives([
            ("new_table_handle", native_new_table_handle as RawSafeNative),
            ("add_box", native_add_box),
            ("borrow_box", native_borrow_box),
            ("borrow_box_mut", native_borrow_box),
            ("remove_box", native_remove_box),
            ("contains_box", native_contains_box),
            ("destroy_empty_box", native_destroy_empty_box),
            ("drop_unchecked_box", native_drop_unchecked_box),
            ("new_table_iter", native_new_table_iter),
            ("prepare_box", native_prepare_box),
            ("next_box", native_next_box),
            ("new_table_iter_mut", native_new_table_iter),
            ("prepare_box_mut", native_prepare_box),
            ("next_box_mut", native_next_box),
        ])
        .map(|(func_name, func)| {
            (
                table_addr,
                Identifier::new("table").unwrap(),
                Identifier::new(func_name).unwrap(),
                func,
            )
        })
        .collect()
}

fn native_new_table_handle(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 2);
    assert_eq!(_arguments.len(), 0);

    context.charge(gas_params.new_table_handle_base)?;

    let table_context = context.extensions().get::<NativeTableContext>();
    let mut table_data = table_context.table_data.borrow_mut();

    // Take the transaction hash provided by the environment, combine it with the # of tables
    // produced so far, sha256 this to produce a unique handle. Given the txn hash
    // is unique, this should create a unique and deterministic global id with native prefix.
    let mut digest = Sha3_256::new();
    let table_len = table_data.new_tables.len() as u32; // cast usize to u32 to ensure same length
    Digest::update(&mut digest, UID_PREFIX);
    Digest::update(&mut digest, table_context.session_id);
    Digest::update(&mut digest, table_len.to_be_bytes());
    let bytes = digest.finalize().to_vec();
    let handle = AccountAddress::from_bytes(&bytes[0..AccountAddress::LENGTH])
        .map_err(|_| partial_extension_error("Unable to create table handle"))?;
    let key_type = context.type_to_type_tag(&ty_args[0])?;
    let value_type = context.type_to_type_tag(&ty_args[1])?;
    assert!(table_data
        .new_tables
        .insert(TableHandle(handle), TableInfo::new(key_type, value_type))
        .is_none());

    Ok(smallvec![Value::address(handle)])
}

fn native_add_box(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 3);

    context.charge(gas_params.add_box_base)?;

    let table_context = context.extensions().get::<NativeTableContext>();
    let mut table_data = table_context.table_data.borrow_mut();

    let val = arguments.pop_back().unwrap();
    let key = arguments.pop_back().unwrap();
    let handle = get_table_handle(&safely_pop_arg!(arguments, StructRef))?;

    let table = table_data.get_or_create_table(context, handle, &ty_args[0], &ty_args[2])?;

    let key_bytes = serialize(&table.key_layout, &key)?;
    let key_cost = gas_params.add_box_per_byte_serialized * NumBytes::new(key_bytes.len() as u64);

    let (gv, loaded) = table.get_or_create_global_value(table_context, key_bytes)?;

    let res = match gv.move_to(val) {
        Ok(_) => Ok(smallvec![]),
        Err(_) => Err(SafeNativeError::Abort {
            abort_code: ALREADY_EXISTS,
        }),
    };

    drop(table_data);

    // TODO(Gas): Figure out a way to charge this earlier.
    context.charge(key_cost)?;
    charge_load_cost(context, loaded)?;

    res
}

fn native_borrow_box(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 2);

    context.charge(gas_params.borrow_box_base)?;

    let table_context = context.extensions().get::<NativeTableContext>();
    let mut table_data = table_context.table_data.borrow_mut();

    let key = arguments.pop_back().unwrap();
    let handle = get_table_handle(&safely_pop_arg!(arguments, StructRef))?;

    let table = table_data.get_or_create_table(context, handle, &ty_args[0], &ty_args[2])?;

    let key_bytes = serialize(&table.key_layout, &key)?;
    let key_cost =
        gas_params.borrow_box_per_byte_serialized * NumBytes::new(key_bytes.len() as u64);

    let (gv, loaded) = table.get_or_create_global_value(table_context, key_bytes)?;

    let res = match gv.borrow_global() {
        Ok(ref_val) => Ok(smallvec![ref_val]),
        Err(_) => Err(SafeNativeError::Abort {
            abort_code: NOT_FOUND,
        }),
    };

    drop(table_data);

    // TODO(Gas): Figure out a way to charge this earlier.
    context.charge(key_cost)?;
    charge_load_cost(context, loaded)?;

    res
}

fn native_contains_box(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 2);

    context.charge(gas_params.contains_box_base)?;

    let table_context = context.extensions().get::<NativeTableContext>();
    let mut table_data = table_context.table_data.borrow_mut();

    let key = arguments.pop_back().unwrap();
    let handle = get_table_handle(&safely_pop_arg!(arguments, StructRef))?;

    let table = table_data.get_or_create_table(context, handle, &ty_args[0], &ty_args[2])?;

    let key_bytes = serialize(&table.key_layout, &key)?;
    let key_cost =
        gas_params.contains_box_per_byte_serialized * NumBytes::new(key_bytes.len() as u64);

    let (gv, loaded) = table.get_or_create_global_value(table_context, key_bytes)?;

    let exists = Value::bool(gv.exists()?);

    drop(table_data);

    // TODO(Gas): Figure out a way to charge this earlier.
    context.charge(key_cost)?;
    charge_load_cost(context, loaded)?;

    Ok(smallvec![exists])
}

fn native_remove_box(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 2);

    context.charge(gas_params.remove_box_base)?;

    let table_context = context.extensions().get::<NativeTableContext>();
    let mut table_data = table_context.table_data.borrow_mut();

    let key = arguments.pop_back().unwrap();
    let handle = get_table_handle(&safely_pop_arg!(arguments, StructRef))?;

    let table = table_data.get_or_create_table(context, handle, &ty_args[0], &ty_args[2])?;

    let key_bytes = serialize(&table.key_layout, &key)?;
    let key_cost =
        gas_params.remove_box_per_byte_serialized * NumBytes::new(key_bytes.len() as u64);

    let (gv, loaded) = table.get_or_create_global_value(table_context, key_bytes)?;

    let res = match gv.move_from() {
        Ok(val) => Ok(smallvec![val]),
        Err(_) => Err(SafeNativeError::Abort {
            abort_code: NOT_FOUND,
        }),
    };

    drop(table_data);

    // TODO(Gas): Figure out a way to charge this earlier.
    context.charge(key_cost)?;
    charge_load_cost(context, loaded)?;

    res
}

fn native_destroy_empty_box(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 1);

    context.charge(gas_params.destroy_empty_box_base)?;

    let table_context = context.extensions().get::<NativeTableContext>();
    let mut table_data = table_context.table_data.borrow_mut();

    let handle = get_table_handle(&safely_pop_arg!(arguments, StructRef))?;
    // TODO: Can the following line be removed?
    table_data.get_or_create_table(context, handle, &ty_args[0], &ty_args[2])?;

    assert!(table_data.removed_tables.insert(handle));

    Ok(smallvec![])
}

fn native_drop_unchecked_box(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 1);

    context.charge(gas_params.drop_unchecked_box_base)?;

    Ok(smallvec![])
}

fn native_new_table_iter(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 4);

    context.charge(gas_params.new_table_iter_base)?;

    let order = Order::try_from(safely_pop_arg!(arguments, u8) as i32)
        .map_err(|_| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?;
    let end_bytes = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    let start_bytes = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    // convert vector start end args into option iterator arguments
    let start_option: Option<&[u8]> = if start_bytes.is_empty() {
        None
    } else {
        Some(start_bytes.as_ref())
    };
    let end_option: Option<&[u8]> = if end_bytes.is_empty() {
        None
    } else {
        Some(end_bytes.as_ref())
    };

    let handle = get_table_handle(&safely_pop_arg!(arguments, StructRef))?;

    // create iterator and store this to table context
    let changes = iter_table_changes(
        context,
        handle,
        &ty_args[0],
        &ty_args[2],
        start_option,
        end_option,
        order,
    )?;

    // charge gas cost for sorting
    context
        .charge(NumArgs::new(changes.len() as u64) * gas_params.new_table_iter_per_item_sorted)?;

    let table_context = context.extensions_mut().get_mut::<NativeTableContext>();
    let iterator_id = table_context
        .resolver
        .create_iterator(&handle, start_option, end_option, order)
        .map_err(|err| {
            partial_extension_error(format!("remote table resolver failure: {}", err))
        })?;

    let mut iterators = table_context.iterators.borrow_mut();
    let context_iterator_id = iterators.len();
    iterators.push(TableIter {
        iterator_id,
        handle,
        changes,
        next: None,
        order,
    });

    // Creating fresh GlobalValue to create local reference to the iterator
    //
    // Self::Fresh { fields } => Ok(ValueImpl::ContainerRef(ContainerRef::Local(
    //   Container::Struct(Rc::clone(fields)),
    // ))),
    //
    let iter_value = Value::struct_(Struct::pack(vec![Value::u64(context_iterator_id as u64)]));

    let mut gv = GlobalValue::none();
    gv.move_to(iter_value)
        .map_err(|_| partial_extension_error("failed to create iterator"))?;

    Ok(smallvec![gv.borrow_global()?])
}

/// Check the `next_key` exist or not and store
/// the computed `next` to the `table_context.next`
/// for the function `next_box`.
fn native_prepare_box(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 1);

    context.charge(gas_params.prepare_box_base)?;

    let iterator_id = get_iterator_id(&safely_pop_arg!(arguments, StructRef))? as usize;

    loop {
        let ((next_key, loaded), handle) = get_next_key_with_table_handle(context, iterator_id)?;
        charge_load_cost(context, loaded)?;

        if next_key.is_none() {
            return Ok(smallvec![Value::bool(false)]);
        }

        let key_bytes = next_key.unwrap();
        let (next, loaded, serialized) =
            load_table_entry(context, handle, &ty_args[0], &ty_args[2], key_bytes)?;
        charge_load_cost(context, loaded)?;
        if let Some(num_bytes) = serialized {
            context.charge(gas_params.prepare_box_per_byte_serialized * num_bytes)?;
        }

        if next.is_some() {
            set_next(context, iterator_id, next);
            return Ok(smallvec![Value::bool(true)]);
        }
    }
}

/// Return `iterator.next` which was computed from
/// the function `prepare_box`.
fn native_next_box(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.table;

    assert_eq!(ty_args.len(), 3);
    assert_eq!(arguments.len(), 1);

    context.charge(gas_params.next_box_base)?;

    let iterator_id = get_iterator_id(&safely_pop_arg!(arguments, StructRef))? as usize;

    let table_context = context.extensions().get::<NativeTableContext>();
    let mut iterators = table_context.iterators.borrow_mut();
    let iterator = iterators.get_mut(iterator_id).unwrap();
    let (key, value) = iterator
        .next
        .take()
        .ok_or_else(|| partial_extension_error("next_box called without prepare_box"))?;

    iterator.next = None;

    Ok(smallvec![key, value])
}

// =========================================================================================
// Helpers

fn get_table_handle(table: &StructRef) -> PartialVMResult<TableHandle> {
    let handle = table
        .borrow_field(HANDLE_FIELD_INDEX)?
        .value_as::<Reference>()?
        .read_ref()?
        .value_as::<AccountAddress>()?;
    Ok(TableHandle(handle))
}

fn get_iterator_id(table_iter: &StructRef) -> PartialVMResult<u64> {
    let iterator_id = table_iter
        .borrow_field(ITERATOR_ID_FIELD_INDEX)?
        .value_as::<Reference>()?
        .read_ref()?
        .value_as::<u64>()?;
    Ok(iterator_id)
}

fn serialize(layout: &MoveTypeLayout, val: &Value) -> PartialVMResult<Vec<u8>> {
    val.simple_serialize(layout)
        .ok_or_else(|| partial_extension_error("cannot serialize table key or value"))
}

fn deserialize(layout: &MoveTypeLayout, bytes: &[u8]) -> PartialVMResult<Value> {
    Value::simple_deserialize(bytes, layout)
        .ok_or_else(|| partial_extension_error("cannot deserialize table key or value"))
}

fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}

fn get_type_layout(context: &NativeContext, ty: &Type) -> PartialVMResult<MoveTypeLayout> {
    context.type_to_type_layout(ty)
}

fn iter_table_changes(
    context: &NativeContext,
    handle: TableHandle,
    key_type: &Type,
    value_type: &Type,
    start: Option<&[u8]>,
    end: Option<&[u8]>,
    order: Order,
) -> PartialVMResult<BTreeSet<Vec<u8>>> {
    let table_context = context.extensions().get::<NativeTableContext>();
    let mut table_data = table_context.table_data.borrow_mut();
    let table = table_data.get_or_create_table(context, handle, key_type, value_type)?;

    // change set iterator
    let bounds = range_bounds(start, end);

    // BTreeMap.range panics if range is start > end.
    // However, this cases represent just empty range and we treat it as such.
    match (bounds.start_bound(), bounds.end_bound()) {
        (Bound::Included(start), Bound::Excluded(end)) if start > end => {
            return Ok(BTreeSet::default());
        }
        _ => {}
    }

    let content_iter = table.content.range(bounds);
    let mut entries: BTreeSet<Vec<u8>> = BTreeSet::default();
    match order {
        Order::Ascending => {
            for (key, _) in content_iter {
                entries.insert(key.to_vec());
            }
        }
        Order::Descending => {
            for (key, _) in content_iter.rev() {
                entries.insert(key.to_vec());
            }
        }
    };

    Ok(entries)
}

fn set_next(context: &mut NativeContext, iterator_id: usize, next: Option<(Value, Value)>) {
    let table_context = context.extensions().get::<NativeTableContext>();
    let mut iterators = table_context.iterators.borrow_mut();
    let iterator = iterators.get_mut(iterator_id);
    assert!(iterator.is_some());

    iterator.unwrap().next = next;
}

#[allow(clippy::type_complexity)]
fn get_next_key_with_table_handle(
    context: &mut NativeContext,
    iterator_id: usize,
) -> PartialVMResult<((Option<Vec<u8>>, Option<Option<NumBytes>>), TableHandle)> {
    let table_context = context.extensions_mut().get_mut::<NativeTableContext>();
    let mut iterators = table_context.iterators.borrow_mut();
    let iterator = iterators.get_mut(iterator_id);

    assert!(iterator.is_some());
    let iterator = iterator.unwrap();

    let res = iterator.load_next_key(table_context.resolver)?;
    Ok((res, iterator.handle))
}

#[allow(clippy::type_complexity)]
fn load_table_entry(
    context: &mut NativeContext,
    handle: TableHandle,
    key_type: &Type,
    value_type: &Type,
    key_bytes: Vec<u8>,
) -> PartialVMResult<(
    Option<(Value, Value)>,
    Option<Option<NumBytes>>,
    Option<NumBytes>,
)> {
    let table_context = context.extensions().get::<NativeTableContext>();
    let mut table_data = table_context.table_data.borrow_mut();
    let table = table_data.get_or_create_table(context, handle, key_type, value_type)?;
    let key_layout = table.key_layout.clone();

    let (gv, loaded) = table.get_or_create_global_value(table_context, key_bytes.clone())?;
    let (key_value, serialized) = if gv.exists()? {
        let key = deserialize(&key_layout, &key_bytes)?;
        let value = gv.borrow_global()?;
        (
            Some((key, value)),
            Some(NumBytes::new(key_bytes.len() as u64)),
        )
    } else {
        (None, None)
    };

    Ok((key_value, loaded, serialized))
}

fn range_bounds(start: Option<&[u8]>, end: Option<&[u8]>) -> impl RangeBounds<Vec<u8>> {
    (
        start.map_or(Bound::Unbounded, |x| Bound::Included(x.to_vec())),
        end.map_or(Bound::Unbounded, |x| Bound::Excluded(x.to_vec())),
    )
}

fn charge_load_cost(
    context: &mut SafeNativeContext,
    loaded: Option<Option<NumBytes>>,
) -> SafeNativeResult<()> {
    let gas_params = &context.native_gas_params.table;

    match loaded {
        Some(Some(num_bytes)) => context
            .charge(gas_params.common_load_base + gas_params.common_load_per_byte * num_bytes),
        Some(None) => context.charge(gas_params.common_load_base + gas_params.common_load_failure),
        None => Ok(()),
    }
}
