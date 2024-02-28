use crate::mocks::{BlankAPIImpl, BlankTableViewImpl};
use initia_move_natives::{
    account::NativeAccountContext, block::NativeBlockContext, code::NativeCodeContext,
    cosmos::NativeCosmosContext, event::NativeEventContext, oracle::NativeOracleContext,
    query::NativeQueryContext, staking::NativeStakingContext, table::NativeTableContext,
    transaction_context::NativeTransactionContext,
};
use move_unit_test;
use move_vm_runtime::native_extensions::NativeContextExtensions;
use once_cell::sync::Lazy;

static mut BLANK_TABLE_RESOLVER: BlankTableViewImpl = BlankTableViewImpl;
static MOCK_API: Lazy<BlankAPIImpl> = Lazy::new(BlankAPIImpl::new);

pub fn configure_for_unit_test() {
    move_unit_test::extensions::set_extension_hook(Box::new(unit_test_extensions_hook))
}

fn unit_test_extensions_hook(exts: &mut NativeContextExtensions) {
    exts.add(NativeAccountContext::new(&MOCK_API.account_api, 1));
    exts.add(NativeTableContext::new([0; 32], unsafe {
        &mut BLANK_TABLE_RESOLVER
    }));
    exts.add(NativeBlockContext::new(0, 0));
    exts.add(NativeCodeContext::default());
    exts.add(NativeStakingContext::new(&MOCK_API.staking_api));
    exts.add(NativeCosmosContext::default());
    exts.add(NativeTransactionContext::new([0; 32], [0; 32]));
    exts.add(NativeEventContext::default());
    exts.add(NativeOracleContext::new(&MOCK_API.oracle_api));
    exts.add(NativeQueryContext::new(&MOCK_API.query_api));
}
