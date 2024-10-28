use std::sync::Arc;

use get_size::GetSize;

#[allow(dead_code)]
#[derive(GetSize)]
pub enum Code<D, V> {
    /// Deserialized code, not yet verified with bytecode verifier.
    Deserialized(Arc<D>),
    /// Fully-verified code.
    Verified(Arc<V>),
}

#[derive(GetSize)]
pub struct ModuleCode<DC, VC, E, V> {
    /// Module's code, either deserialized or verified.
    pub code: Code<DC, VC>,
    /// Module's extension - any additional metadata associated with this module.
    pub extension: Arc<E>,
    /// Version of the code (e.g., which transaction within the block published this module).
    version: V,
}
