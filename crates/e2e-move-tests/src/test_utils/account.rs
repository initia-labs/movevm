use move_core_types::account_address::AccountAddress;

pub fn generate_account(literal: &str) -> AccountAddress {
    AccountAddress::from_hex_literal(literal).expect("account should be created")
}
