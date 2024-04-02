use crate::json_event::JsonEvent;
use crate::serde_helper::vec_bytes;

use move_core_types::identifier::{IdentStr, Identifier};
use move_core_types::language_storage::{ModuleId, TypeTag};

use serde::{Deserialize, Serialize};

/// Call a Move script function.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ViewFunction {
    module: ModuleId,
    function: Identifier,
    ty_args: Vec<TypeTag>,
    #[serde(with = "vec_bytes")]
    args: Vec<Vec<u8>>,

    // whether the args are json encoded
    is_json: bool,
}

impl ViewFunction {
    pub fn new(
        module: ModuleId,
        function: Identifier,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
        is_json: bool,
    ) -> Self {
        ViewFunction {
            module,
            function,
            ty_args,
            args,
            is_json,
        }
    }

    pub fn module(&self) -> &ModuleId {
        &self.module
    }

    pub fn function(&self) -> &IdentStr {
        &self.function
    }

    pub fn ty_args(&self) -> &Vec<TypeTag> {
        &self.ty_args
    }

    pub fn args(&self) -> &Vec<Vec<u8>> {
        &self.args
    }

    pub fn into_inner(self) -> (ModuleId, Identifier, Vec<TypeTag>, Vec<Vec<u8>>) {
        (self.module, self.function, self.ty_args, self.args)
    }

    pub fn is_json(&self) -> bool {
        self.is_json
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ViewOutput {
    ret: String,
    events: Vec<JsonEvent>,

    /// The amount of gas used during execution.
    gas_used: u64,
}

impl ViewOutput {
    pub fn new(ret: String, events: Vec<JsonEvent>, gas_used: u64) -> Self {
        ViewOutput {
            ret,
            events,
            gas_used,
        }
    }

    pub fn ret(&self) -> &String {
        &self.ret
    }

    pub fn events(&self) -> &Vec<JsonEvent> {
        &self.events
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }
}
