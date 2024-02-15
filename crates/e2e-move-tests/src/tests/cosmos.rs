use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use initia_types::cosmos::{
    CosmosCoin, CosmosMessage, DistributionMessage, IBCFee, IBCHeight, IBCMessage, MoveMessage,
    StakingMessage, StargateMessage,
};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;
use sha3::{Digest, Sha3_256};

const STAKING_SYMBOL: &[u8] = b"ustake";
const FEE_A_SYMBOL: &[u8] = b"ufoo";
const FEE_B_SYMBOL: &[u8] = b"ubar";
const COLLECTION_NAME: &[u8] = b"collection";

fn staking_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(STAKING_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

fn fee_a_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(FEE_A_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

fn fee_b_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(FEE_B_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

fn collection_addr() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(COLLECTION_NAME);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

type TestInput<'a> = (
    AccountAddress,
    &'a str,
    Vec<TypeTag>,
    Vec<Vec<u8>>,
    ExpectedOutput,
);

fn run_tests(tests: Vec<TestInput>) {
    let mut h = MoveHarness::new();

    h.initialize();

    for (sender, entry, ty_args, args, exp_output) in tests {
        let vm_output = h.run_entry_function(
            vec![sender],
            str::parse(entry).unwrap(),
            ty_args.clone(),
            args,
        );
        exp_output.check_execute_output(&vm_output);

        if let Ok(output) = vm_output {
            h.commit(output, true);
        }
    }
}

#[test]
fn test_cosmos_delegate() {
    let mut tests = vec![];
    let delegator_address = AccountAddress::random();
    let validator_address = "validator".to_string();
    let metadata = staking_metadata();
    let amount = 100u64;

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_delegate = (
        delegator_address,
        "0x1::cosmos::delegate",
        vec![],
        vec![
            bcs::to_bytes(validator_address.as_bytes()).unwrap(),
            metadata.to_vec(),
            amount.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::Staking(StakingMessage::Delegate {
                delegator_address,
                validator_address,
                amount: CosmosCoin { amount, metadata },
            })]),
        ),
    );
    tests.push(test_delegate);

    run_tests(tests);
}

#[test]
fn test_cosmos_fund_community_pool() {
    let mut tests = vec![];
    let sender_address = AccountAddress::random();
    let metadata = staking_metadata();
    let amount = 100u64;

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_fund_community_pool = (
        sender_address,
        "0x1::cosmos::fund_community_pool",
        vec![],
        vec![metadata.to_vec(), amount.to_le_bytes().to_vec()],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::Distribution(
                DistributionMessage::FundCommunityPool {
                    sender_address,
                    amount: CosmosCoin { amount, metadata },
                },
            )]),
        ),
    );
    tests.push(test_fund_community_pool);

    run_tests(tests);
}

#[test]
fn test_cosmos_transfer() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let receiver = "receiver".to_string();
    let metadata = staking_metadata();
    let amount = 100u64;
    let source_port = "port".to_string();
    let source_channel = "channel".to_string();
    let revision_number = 1u64;
    let revision_height = 2u64;
    let timeout_timestamp = 100u64;
    let memo = "memo".to_string();

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_transfer = (
        sender,
        "0x1::cosmos::transfer",
        vec![],
        vec![
            bcs::to_bytes(receiver.as_bytes()).unwrap(),
            metadata.to_vec(),
            amount.to_le_bytes().to_vec(),
            bcs::to_bytes(source_port.as_bytes()).unwrap(),
            bcs::to_bytes(source_channel.as_bytes()).unwrap(),
            revision_number.to_le_bytes().to_vec(),
            revision_height.to_le_bytes().to_vec(),
            timeout_timestamp.to_le_bytes().to_vec(),
            bcs::to_bytes(memo.as_bytes()).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::IBC(IBCMessage::Transfer {
                source_port,
                source_channel,
                token: CosmosCoin { amount, metadata },
                sender,
                receiver,
                timeout_height: IBCHeight {
                    revision_height,
                    revision_number,
                },
                timeout_timestamp,
                memo,
            })]),
        ),
    );
    tests.push(test_transfer);

    run_tests(tests);
}

#[test]
fn test_cosmos_nft_transfer() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let receiver = "receiver".to_string();
    let collection = collection_addr();
    let token_ids = vec!["id1".to_string(), "id2".to_string()];
    let source_port = "port".to_string();
    let source_channel = "channel".to_string();
    let revision_number = 1u64;
    let revision_height = 2u64;
    let timeout_timestamp = 100u64;
    let memo = "memo".to_string();

    let test_create_collection = (
        AccountAddress::ONE,
        "0x1::simple_nft::create_collection",
        vec![],
        vec![
            bcs::to_bytes(&b"Test Collection".to_vec()).unwrap(),
            vec![0],
            bcs::to_bytes(&COLLECTION_NAME.to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_create_collection);

    let test_nft_transfer = (
        sender,
        "0x1::cosmos::nft_transfer",
        vec![],
        vec![
            bcs::to_bytes(receiver.as_bytes()).unwrap(),
            collection.to_vec(),
            bcs::to_bytes(&token_ids).unwrap(),
            bcs::to_bytes(source_port.as_bytes()).unwrap(),
            bcs::to_bytes(source_channel.as_bytes()).unwrap(),
            revision_number.to_le_bytes().to_vec(),
            revision_height.to_le_bytes().to_vec(),
            timeout_timestamp.to_le_bytes().to_vec(),
            bcs::to_bytes(memo.as_bytes()).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::IBC(IBCMessage::NFTTransfer {
                source_port,
                source_channel,
                collection,
                token_ids,
                sender,
                receiver,
                timeout_height: IBCHeight {
                    revision_height,
                    revision_number,
                },
                timeout_timestamp,
                memo,
            })]),
        ),
    );
    tests.push(test_nft_transfer);

    run_tests(tests);
}

#[test]
fn test_cosmos_pay_fee() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let source_port = "port".to_string();
    let source_channel = "channel".to_string();
    let recv_fee_metadata = staking_metadata();
    let recv_fee_amount = 100u64;
    let ack_fee_metadata = fee_a_metadata();
    let ack_fee_amount = 200u64;
    let timeout_fee_metadata = fee_b_metadata();
    let timeout_fee_amount = 300u64;

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Fee A Denom".to_vec()).unwrap(),
            bcs::to_bytes(FEE_A_SYMBOL).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Fee B Denom".to_vec()).unwrap(),
            bcs::to_bytes(FEE_B_SYMBOL).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_pay_fee = (
        sender,
        "0x1::cosmos::pay_fee",
        vec![],
        vec![
            bcs::to_bytes(source_port.as_bytes()).unwrap(),
            bcs::to_bytes(source_channel.as_bytes()).unwrap(),
            recv_fee_metadata.to_vec(),
            recv_fee_amount.to_le_bytes().to_vec(),
            ack_fee_metadata.to_vec(),
            ack_fee_amount.to_le_bytes().to_vec(),
            timeout_fee_metadata.to_vec(),
            timeout_fee_amount.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::IBC(IBCMessage::PayFee {
                signer: sender,
                source_port,
                source_channel,
                fee: IBCFee {
                    recv_fee: CosmosCoin {
                        metadata: recv_fee_metadata,
                        amount: recv_fee_amount,
                    },
                    ack_fee: CosmosCoin {
                        metadata: ack_fee_metadata,
                        amount: ack_fee_amount,
                    },
                    timeout_fee: CosmosCoin {
                        metadata: timeout_fee_metadata,
                        amount: timeout_fee_amount,
                    },
                },
            })]),
        ),
    );
    tests.push(test_pay_fee);

    run_tests(tests);
}

#[test]
fn test_initiate_token_deposit() {
    let mut tests = vec![];
    let bridge_id = 10u64;
    let sender = AccountAddress::random();
    let to = AccountAddress::random();
    let metadata = staking_metadata();
    let amount = 100u64;
    let data = vec![1, 2, 3, 4];

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_pay_fee = (
        sender,
        "0x1::cosmos::initiate_token_deposit",
        vec![],
        vec![
            bridge_id.to_le_bytes().to_vec(),
            to.to_vec(),
            metadata.to_vec(),
            amount.to_le_bytes().to_vec(),
            bcs::to_bytes(&data.to_vec()).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::OPinit(
                initia_types::cosmos::OPinitMessage::InitiateTokenDeposit {
                    bridge_id,
                    sender_address: sender,
                    to_address: to,
                    amount: CosmosCoin { amount, metadata },
                    data,
                },
            )]),
        ),
    );
    tests.push(test_pay_fee);

    run_tests(tests);
}

#[test]
fn test_cosmos_move_execute() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let module_address = AccountAddress::random();
    let module_name = "module_name".to_string();
    let function_name = "function_name".to_string();
    let type_arg1 = "type_arg1".to_string();
    let type_arg2 = "type_arg2".to_string();
    let arg1 = vec![1, 2, 3];
    let arg2 = vec![4, 5, 6];

    let test_move_execute = (
        sender,
        "0x1::cosmos::move_execute",
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(module_name.as_bytes()).unwrap(),
            bcs::to_bytes(function_name.as_bytes()).unwrap(),
            bcs::to_bytes(&vec![type_arg1.as_bytes(), type_arg2.as_bytes()]).unwrap(),
            bcs::to_bytes(&vec![arg1.to_vec(), arg2.to_vec()]).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::Move(MoveMessage::Execute {
                sender,
                module_address,
                module_name,
                function_name,
                type_args: vec![type_arg1, type_arg2],
                args: vec![arg1, arg2],
            })]),
        ),
    );
    tests.push(test_move_execute);

    run_tests(tests);
}

#[test]
fn test_cosmos_move_script() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let code_bytes = vec![1, 2, 3, 4, 5];
    let type_arg1 = "type_arg1".to_string();
    let type_arg2 = "type_arg2".to_string();
    let arg1 = vec![1, 2, 3];
    let arg2 = vec![4, 5, 6];

    let test_move_script = (
        sender,
        "0x1::cosmos::move_script",
        vec![],
        vec![
            bcs::to_bytes(&code_bytes.to_vec()).unwrap(),
            bcs::to_bytes(&vec![type_arg1.as_bytes(), type_arg2.as_bytes()]).unwrap(),
            bcs::to_bytes(&vec![arg1.to_vec(), arg2.to_vec()]).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::Move(MoveMessage::Script {
                sender,
                code_bytes,
                type_args: vec![type_arg1, type_arg2],
                args: vec![arg1, arg2],
            })]),
        ),
    );
    tests.push(test_move_script);

    run_tests(tests);
}

#[test]
fn test_cosmos_stargate() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let path = "path".to_string();
    let data = "data".to_string();

    let test_stargate = (
        sender,
        "0x1::cosmos::stargate",
        vec![],
        vec![
            bcs::to_bytes(path.as_bytes()).unwrap(),
            bcs::to_bytes(data.as_bytes()).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage::Stargate(StargateMessage {
                sender,
                path,
                data: data.as_bytes().to_vec(),
            })]),
        ),
    );
    tests.push(test_stargate);

    run_tests(tests);
}
