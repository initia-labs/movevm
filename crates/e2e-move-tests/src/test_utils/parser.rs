use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use std::str::FromStr;
use thiserror::Error;

/// A common result to be returned to users
pub type ParserResult = Result<String, String>;

/// A common result to remove need for typing `Result<T, ParserError>`
pub type ParserTypedResult<T> = Result<T, ParserError>;

/// CLI Errors for reporting through telemetry and outputs
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Invalid arguments: {0}")]
    CommandArgumentError(String),
    #[error("Unable to parse '{0}': error: {1}")]
    UnableToParse(&'static str, String),
}

/// Identifier of a module member (function or struct).
#[derive(Debug, Clone)]
pub struct MemberId {
    pub module_id: ModuleId,
    pub member_id: Identifier,
}

pub fn load_account_arg(str: &str) -> Result<AccountAddress, ParserError> {
    if str.starts_with("0x") {
        AccountAddress::from_hex_literal(str).map_err(|err| {
            ParserError::CommandArgumentError(format!("Failed to parse AccountAddress {}", err))
        })
    } else if let Ok(account_address) = AccountAddress::from_str(str) {
        Ok(account_address)
    } else {
        Err(ParserError::CommandArgumentError(
            "'Failed to parse AccountAddress'".to_string(),
        ))
    }
}

fn parse_member_id(function_id: &str) -> ParserTypedResult<MemberId> {
    let ids: Vec<&str> = function_id.split_terminator("::").collect();
    if ids.len() != 3 {
        return Err(ParserError::CommandArgumentError(
            "FunctionId is not well formed.  Must be of the form <address>::<module>::<function>"
                .to_string(),
        ));
    }

    let address = load_account_arg(ids.first().unwrap())?;

    let module = Identifier::from_str(ids.get(1).unwrap())
        .map_err(|err| ParserError::UnableToParse("Module Name", err.to_string()))?;
    let member_id = Identifier::from_str(ids.get(2).unwrap())
        .map_err(|err| ParserError::UnableToParse("Member Name", err.to_string()))?;
    let module_id = ModuleId::new(address, module);
    Ok(MemberId {
        module_id,
        member_id,
    })
}

impl FromStr for MemberId {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_member_id(s)
    }
}
