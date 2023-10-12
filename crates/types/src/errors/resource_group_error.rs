use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid resource group scope: {0}")]
pub struct ResourceGroupScopeError(pub String);
