use crate::MoveHarness;
use move_core_types::u256::U256;

#[test]
fn test_oracle() {
    let mut h = MoveHarness::new();

    h.initialize();

    let pair_id = "BITCOIN/USD".to_string();
    let price: U256 = U256::from(1123451219823412_u64);
    let updated_at = 100_u64;
    let decimals = 8_u64;

    h.api
        .oracle_api
        .set_oracle_price(pair_id.clone().into_bytes(), price, updated_at, decimals);

    let view_fn = h.create_view_function(
        str::parse("0x1::oracle::get_price").unwrap(),
        vec![],
        vec![bcs::to_bytes(&pair_id).unwrap()],
    );

    let view_output = h
        .run_view_function(view_fn)
        .expect("get_price should success");
    assert_eq!(view_output, "[\"1123451219823412\",\"100\",\"8\"]");
}
