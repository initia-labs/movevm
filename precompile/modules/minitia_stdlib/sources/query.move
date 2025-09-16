module minitia_std::query {
    use minitia_std::string::{Self, String};
    use minitia_std::json;
    use minitia_std::option;

    struct ProposalRequest has copy, drop {
        proposal_id: u64
    }

    struct ProposalResponse has copy, drop {
        id: u64,
        title: String,
        summary: String,
        status: String,
        submit_time: String,
        emergency: bool
    }

    fun unmarshal_proposal_response(response: vector<u8>): ProposalResponse {
        json::unmarshal_json_value<ProposalResponse>(
            option::destroy_some(
                json::get_elem(
                    &json::unmarshal<json::JSONObject>(response),
                    string::utf8(b"proposal")
                )
            )
        )
    }

    #[view]
    public fun get_proposal(proposal_id: u64): (u64, String, String, String) {
        let response =
            query_stargate(
                b"/initia.gov.v1.Query/Proposal",
                json::marshal(&ProposalRequest { proposal_id })
            );

        let res = unmarshal_proposal_response(response);
        (res.id, res.title, res.summary, res.status)
    }

    #[view]
    public fun get_proposal_status(proposal_id: u64): (u64, String, String, bool) {
        let response =
            query_stargate(
                b"/initia.gov.v1.Query/Proposal",
                json::marshal(&ProposalRequest { proposal_id })
            );
        let res = unmarshal_proposal_response(response);
        (res.id, res.status, res.submit_time, res.emergency)
    }

    /// query_custom examples are in minitia_stdlib::address module
    native public fun query_custom(name: vector<u8>, data: vector<u8>): vector<u8>;
    native public fun query_stargate(path: vector<u8>, data: vector<u8>): vector<u8>;

    #[test_only]
    native public fun set_query_response(
        path_or_name: vector<u8>, data: vector<u8>, response: vector<u8>
    );

    #[test_only]
    native public fun unset_query_response(
        path_or_name: vector<u8>, data: vector<u8>
    );

    #[test]
    fun test_query_custom() {
        set_query_response(b"path", b"data123", b"output");

        let res = query_custom(b"path", b"data123");
        assert!(res == b"output", 0);
    }

    #[test]
    fun test_query_stargate() {
        set_query_response(b"path", b"data123", b"output");

        let res = query_stargate(b"path", b"data123");
        assert!(res == b"output", 0);
    }

    #[test]
    fun test_get_proposal() {
        let req = json::marshal(&ProposalRequest { proposal_id: 40 });
        set_query_response(
            b"/initia.gov.v1.Query/Proposal",
            req,
            b"{\"proposal\":{\"id\":\"40\",\"messages\":[{\"@type\":\"/initia.move.v1.MsgGovExecute\",\"authority\":\"init10d07y265gmmuvt4z0w9aw880jnsr700j55nka3\",\"sender\":\"init182yxkv4gqfvz7tjyde6dfgjdr4ldqxklgmf23aju2u3cslnss7ys6dy6w8\",\"module_address\":\"init182yxkv4gqfvz7tjyde6dfgjdr4ldqxklgmf23aju2u3cslnss7ys6dy6w8\",\"module_name\":\"vip\",\"function_name\":\"update_l2_score_contract\",\"type_args\":[],\"args\":[\"HQAAAAAAAAA=\",\"KjB4MzllMTAyZjYxMEEyMzI2MEFkQTA5MzQ1N2ZCN0NkN0MzMTIwM0JCOQ==\"]}],\"status\":\"PROPOSAL_STATUS_PASSED\",\"final_tally_result\":{\"tally_height\":\"2941401\",\"total_staking_power\":\"58870777897045\",\"total_vesting_power\":\"0\",\"v1_tally_result\":{\"yes_count\":\"38322200084968\",\"abstain_count\":\"36826705051\",\"no_count\":\"69745588282\",\"no_with_veto_count\":\"0\"}},\"submit_time\":\"2025-05-21T16:35:32.434456966Z\",\"deposit_end_time\":\"2025-05-22T16:35:32.434456966Z\",\"total_deposit\":[{\"denom\":\"uinit\",\"amount\":\"100000000000\"}],\"voting_start_time\":\"2025-05-21T17:28:48.761350570Z\",\"voting_end_time\":\"2025-05-22T17:28:48.761350570Z\",\"emergency_start_time\":null,\"emergency_next_tally_time\":null,\"metadata\":\"https://forum.initia.xyz/t/update-embr-fun-vip-score-contract-address/188\",\"title\":\"Update Embr.fun VIP Score Contract Address\",\"summary\":\"This proposal is submitted by the Initia Foundation on behalf of Embr. Please see the following statement by the team below.\",\"expedited\":true,\"emergency\":false,\"failed_reason\":\"\"}}"
        );

        let (id, title, summary, status) = get_proposal(40);
        assert!(id == 40, 0);
        assert!(title == string::utf8(b"Update Embr.fun VIP Score Contract Address"), 0);
        assert!(
            summary
                == string::utf8(
                    b"This proposal is submitted by the Initia Foundation on behalf of Embr. Please see the following statement by the team below."
                ),
            0
        );
        assert!(status == string::utf8(b"PROPOSAL_STATUS_PASSED"), 0);
    }

    #[test]
    #[expected_failure(abort_code = 0x1006E, location = Self)]
    fun test_query_unsset() {
        set_query_response(b"path", b"data123", b"output");
        unset_query_response(b"path", b"data123");

        let res = query_custom(b"path", b"data123");
        assert!(res == b"", 0);
    }
}
