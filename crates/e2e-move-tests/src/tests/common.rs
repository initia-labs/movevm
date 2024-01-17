use initia_types::{cosmos::CosmosMessage, message::MessageOutput};
use std::collections::BTreeMap;

use move_core_types::{account_address::AccountAddress, vm_status::VMStatus};

type VMOutput = (VMStatus, MessageOutput, Option<String>);
type StakingDelta = (Vec<u8>, Vec<(AccountAddress, (u64, u64))>);

pub struct ExpectedOutput(pub Vec<ExpectedOutputItem>);

impl ExpectedOutput {
    // for compatibility with previous tests
    pub fn new(
        vm_status: VMStatus,
        response: Option<String>,
        staking_delta: Option<Vec<StakingDelta>>,
        cosmos_messages: Option<Vec<CosmosMessage>>,
    ) -> Self {
        let mut items = vec![ExpectedOutputItem::VMStatusReturn(vm_status)];
        if let Some(s) = response {
            items.push(ExpectedOutputItem::Response(s));
        }

        if let Some(sd) = staking_delta {
            let mut delta_map = BTreeMap::new();
            for (val, deltas_per_val) in sd.into_iter() {
                let deltas = BTreeMap::from_iter(deltas_per_val.into_iter());
                delta_map.insert(val, deltas);
            }

            items.push(ExpectedOutputItem::StakingChange(delta_map));
        }

        if let Some(messages) = cosmos_messages {
            items.push(ExpectedOutputItem::CosmosMessages(messages))
        }

        Self(items)
    }

    pub fn check_execute_output(&self, exec_output: &Result<MessageOutput, VMStatus>) {
        let (vm_status, output): (VMStatus, MessageOutput) = match exec_output {
            Ok(output) => (VMStatus::Executed, output.clone()),
            Err(err) => (err.clone(), MessageOutput::default()),
        };

        self.check_vm_output_internal(&(vm_status, output, None))
    }

    pub fn check_view_output(&self, view_output: &Result<String, VMStatus>) {
        let (vm_status, output) = match view_output.clone() {
            Ok(output) => (VMStatus::Executed, output),
            Err(err) => (err, "".into()),
        };

        self.check_vm_output_internal(&(vm_status, MessageOutput::default(), Some(output)))
    }

    fn check_vm_output_internal(&self, vm_output: &VMOutput) {
        for exp in &self.0 {
            exp.check_output(vm_output);
        }
    }
}

pub enum ExpectedOutputItem {
    VMStatusReturn(VMStatus),
    Response(String),
    StakingChange(BTreeMap<Vec<u8>, BTreeMap<AccountAddress, (u64, u64)>>),
    CosmosMessages(Vec<CosmosMessage>),
}

impl ExpectedOutputItem {
    pub fn check_output(&self, vm_output: &VMOutput) {
        let (status, output, response) = vm_output;
        match self {
            ExpectedOutputItem::VMStatusReturn(exp_status) => {
                println!("got:{:?}, exp:{}", status, exp_status);
                assert!(status == exp_status);
            }
            ExpectedOutputItem::Response(exp_string) => {
                println!("response: {:?}", response);
                assert!(response.is_some());
                assert!(response.as_ref().unwrap() == exp_string);
            }
            ExpectedOutputItem::StakingChange(exp_map) => {
                assert!(
                    *output.staking_change_set().changes() == *exp_map,
                    "expected\n{:?}\n\noutput\n{:?}",
                    exp_map,
                    output.staking_change_set().changes()
                );
            }
            ExpectedOutputItem::CosmosMessages(exp_messages) => {
                assert!(
                    *output.cosmos_messages().inner() == *exp_messages,
                    "expected\n{:?}\n\noutput\n{:?}",
                    exp_messages,
                    output.cosmos_messages().inner()
                );
            }
        };
    }
}
