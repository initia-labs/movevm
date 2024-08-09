// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

macro_rules! define_gas_parameters {
    (
        $params_name: ident,
        $prefix: literal,
        $env: ty => $(.$field: ident)*,
        [$(
            [$name: ident: $ty: ident, $key: tt, $initial: expr $(, $tn: ident)? $(,)?]
        ),* $(,)?]
    ) => {
        #[derive(Debug, Clone)]
        pub struct $params_name {
            $(pub $name : $ty),*
        }

        impl $crate::traits::FromOnChainGasSchedule for $params_name {
            #[allow(unused)]
            fn from_on_chain_gas_schedule(gas_schedule: &std::collections::BTreeMap<String, u64>) -> Result<Self, String> {
                let mut params = $params_name::zeros();

                $(
                    let name = format!("{}.{}", $prefix, $key);
                    params.$name = gas_schedule.get(&name).cloned().ok_or_else(|| format!("Gas parameter {} does not exist.", name))?.into();
                )*

                Ok(params)
            }
        }

        impl $crate::traits::ToOnChainGasSchedule for $params_name {
            #[allow(unused)]
            fn to_on_chain_gas_schedule(&self) -> Vec<(String, u64)> {
                let mut output = vec![];

                $(
                    output.push((format!("{}.{}", $prefix, $key), self.$name.into()));
                )*

                output
            }
        }

        impl $params_name {
            pub fn zeros() -> Self {
                Self {
                    $($name: 0.into()),*
                }
            }

            #[cfg(feature = "testing")]
            pub fn random() -> Self {
                Self {
                    $($name: rand::random::<u64>().into()),*
                }
            }
        }

        impl $crate::traits::InitialGasSchedule for $params_name {
            fn initial() -> Self {
                Self {
                    $($name: $initial.into()),*
                }
            }
        }
    };
}

pub(crate) use define_gas_parameters;
