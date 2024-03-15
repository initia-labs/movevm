module minitia_std::query {
    use minitia_std::string::{Self, String};
    use minitia_std::option;
    use minitia_std::json;
    use minitia_std::simple_json;


    /*
    type QueryProposalResponse struct {
        Proposal *Proposal `protobuf:"bytes,1,opt,name=proposal,proto3" json:"proposal,omitempty"`
    }

    type Proposal struct {
        Id uint64 `protobuf:"varint,1,opt,name=id,proto3" json:"id,omitempty"`
        Messages []*types1.Any `protobuf:"bytes,2,rep,name=messages,proto3" json:"messages,omitempty"`
        Status v1.ProposalStatus `protobuf:"varint,3,opt,name=status,proto3,enum=cosmos.gov.v1.ProposalStatus" json:"status,omitempty"`
        FinalTallyResult *v1.TallyResult `protobuf:"bytes,4,opt,name=final_tally_result,json=finalTallyResult,proto3" json:"final_tally_result,omitempty"`
        SubmitTime *time.Time `protobuf:"bytes,5,opt,name=submit_time,json=submitTime,proto3,stdtime" json:"submit_time,omitempty"`
        DepositEndTime *time.Time `protobuf:"bytes,6,opt,name=deposit_end_time,json=depositEndTime,proto3,stdtime" json:"deposit_end_time,omitempty"`
        TotalDeposit []types.Coin `protobuf:"bytes,7,rep,name=total_deposit,json=totalDeposit,proto3" json:"total_deposit"`
        VotingStartTime *time.Time `protobuf:"bytes,8,opt,name=voting_start_time,json=votingStartTime,proto3,stdtime" json:"voting_start_time,omitempty"`
        VotingEndTime          *time.Time `protobuf:"bytes,9,opt,name=voting_end_time,json=votingEndTime,proto3,stdtime" json:"voting_end_time,omitempty"`
        EmergencyStartTime     *time.Time `protobuf:"bytes,10,opt,name=emergency_start_time,json=emergencyStartTime,proto3,stdtime" json:"emergency_start_time,omitempty"`
        EmergencyNextTallyTime *time.Time `protobuf:"bytes,11,opt,name=emergency_next_tally_time,json=emergencyNextTallyTime,proto3,stdtime" json:"emergency_next_tally_time,omitempty"`
        Metadata string `protobuf:"bytes,12,opt,name=metadata,proto3" json:"metadata,omitempty"`

        Title string `protobuf:"bytes,13,opt,name=title,proto3" json:"title,omitempty"`

        Summary string `protobuf:"bytes,14,opt,name=summary,proto3" json:"summary,omitempty"`

        Proposer string `protobuf:"bytes,15,opt,name=proposer,proto3" json:"proposer,omitempty"`

        Expedited bool `protobuf:"varint,16,opt,name=expedited,proto3" json:"expedited,omitempty"`
        Emergency bool `protobuf:"varint,17,opt,name=emergency,proto3" json:"emergency,omitempty"`

        FailedReason string `protobuf:"bytes,18,opt,name=failed_reason,json=failedReason,proto3" json:"failed_reason,omitempty"`
    }

    */
    #[view]
    public fun get_proposal(proposal_id: u64): (u64, String, String, String) {
        let obj = json::empty();
        let index = json::start_index();
        json::set_object(&mut obj, index, option::none<String>(), 1);
        json::set_int_raw(&mut obj, json::get_next_index(&index, 0), option::some(string::utf8(b"proposal_id")), true, (proposal_id as u256));

        let req = json::stringify(&obj);
        let response = query_stargate(b"/initia.gov.v1.Query/Proposal", *string::bytes(&req));
        let res = json::parse(string::utf8(response));
        let index = json::get_next_index(&index, 0);

        let cindex = json::find(&res, &index, &string::utf8(b"id"));
        let (_, data) = json::unpack_elem(json::borrow(&res, &cindex));
        let (_, id) = json::as_int(data);

        let cindex = json::find(&res, &index, &string::utf8(b"title"));
        let (_, data) = json::unpack_elem(json::borrow(&res, &cindex));
        let title = json::as_string(data);

        let cindex = json::find(&res, &index, &string::utf8(b"summary"));
        let (_, data) = json::unpack_elem(json::borrow(&res, &cindex));
        let summary = json::as_string(data);
        ((id as u64), title, summary, string::utf8(response))
    }

    #[view]
    public fun get_proposal_status(proposal_id: u64): (u64, String, String, bool) {
        let obj = simple_json::empty();
        simple_json::set_object(&mut obj, option::none<String>());
        simple_json::increase_depth(&mut obj);
        simple_json::set_int_raw(&mut obj, option::some(string::utf8(b"proposal_id")), true, (proposal_id as u256));

        let req = json::stringify(simple_json::to_json_object(&obj));
        let res = query_stargate(b"/initia.gov.v1.Query/Proposal", *string::bytes(&req));
        let res = simple_json::from_json_object(json::parse(string::utf8(res)));
        simple_json::increase_depth(&mut res);
        simple_json::increase_depth(&mut res);

        simple_json::find_and_set_index(&mut res, &string::utf8(b"id"));
        let (_, data) = json::unpack_elem(simple_json::borrow(&mut res));
        let (_, id) = json::as_int(data);
        
        simple_json::find_and_set_index(&mut res, &string::utf8(b"status"));
        let (_, data) = json::unpack_elem(simple_json::borrow(&mut res));
        let status = json::as_string(data);

        simple_json::find_and_set_index(&mut res, &string::utf8(b"submit_time"));
        let (_, data) = json::unpack_elem(simple_json::borrow(&mut res));
        let submit_time = json::as_string(data);

        simple_json::find_and_set_index(&mut res, &string::utf8(b"emergency"));
        let (_, data) = json::unpack_elem(simple_json::borrow(&mut res));
        let emergency = json::as_bool(data);
        ((id as u64), status, submit_time, emergency)
    }

    /// query_custom examples are in initia_stdlib::address module
    native public fun query_custom(name: vector<u8>, data: vector<u8>): vector<u8>;
    native public fun query_stargate(path: vector<u8>, data: vector<u8>): vector<u8>;

    #[test_only]
    native public fun set_query_response(path_or_name: vector<u8>, data: vector<u8>, response: vector<u8>);

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
}