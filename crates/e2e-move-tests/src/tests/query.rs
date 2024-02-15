use crate::MoveHarness;
use move_core_types::u256::U256;

#[test]
fn test_query() {
    let mut h = MoveHarness::new();

    h.initialize();

    let proposal_id = 1;

    let view_fn = h.create_view_function(
        str::parse("0x1::query::get_proposal").unwrap(),
        vec![],
        vec![bcs::to_bytes(&proposal_id).unwrap()],
    );

    let view_output = h
        .run_view_function(view_fn)
        .expect("get_proposal should success");
    assert_eq!(view_output, "{\"proposal\":{\"id\":0,\"title\":\"test_proposal\",\"summary\":\"test_proposal_summary\"}}");
}
