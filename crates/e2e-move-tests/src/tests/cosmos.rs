use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use initia_types::cosmos::{
    CosmosCoin, CosmosMessage, DistributionMessage, IBCFee, IBCHeight, IBCMessage, StakingMessage,
};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;
use sha3::{Digest, Sha3_256};

const STAKING_SYMBOL: &[u8] = b"ustake";
const FEE_A_SYMBOL: &[u8] = b"ufoo";
const FEE_B_SYMBOL: &[u8] = b"ubar";

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

fn run_tests(
    tests: Vec<(
        AccountAddress,
        &str,
        Vec<TypeTag>,
        Vec<Vec<u8>>,
        ExpectedOutput,
    )>,
) {
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
            bcs::to_bytes(STAKING_SYMBOL).unwrap(),
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
            bcs::to_bytes(STAKING_SYMBOL).unwrap(),
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
            bcs::to_bytes(STAKING_SYMBOL).unwrap(),
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
            bcs::to_bytes(STAKING_SYMBOL).unwrap(),
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
