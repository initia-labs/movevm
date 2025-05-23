use crate::{ InitiaCLI, InitiaCommand };
use std::{ fs, path::PathBuf };
use anyhow::Context;
use clap::{ Parser, Subcommand };
use movevm::move_api::handler::{ decode_module_bytes, decode_script_bytes, read_module_info };

#[derive(Parser)]
#[command(
    name = "decode",
    about = "Read or Decode Move modules and scripts",
    long_about = "Read or Decode Move modules and prints the result in JSON format"
)]
pub struct Decode {
    #[command(subcommand)]
    pub command: DecodeCommands,
}

#[derive(Subcommand)]
pub enum DecodeCommands {
    #[command(
        name = "read",
        about = "Read Move module info from bytecode",
        long_about = "Read and display basic information about a Move module from its bytecode file.\n\
        Example: initia-move decode read ./build/package/bytecode_modules/my_module.mv"
    )] Read {
        /// Path to the Move module bytecode file
        #[arg(value_name = "FILE")]
        path: String,
    },

    #[command(
        name = "script",
        about = "Decode Move script bytecode",
        long_about = "Decode Move script bytecode and display its ABI (Application Binary Interface).\n\
        Example: initia-move decode script ./build/package/scripts/my_script.mv"
    )] Script {
        /// Path to the Move script bytecode file
        #[arg(value_name = "FILE")]
        path: String,
    },

    #[command(
        name = "module",
        about = "Decode Move module bytecode",
        long_about = "Decode Move module bytecode and display its ABI (Application Binary Interface).\n\
        Example: initia-move decode module ./build/package/bytecode_modules/my_module.mv"
    )] Module {
        /// Path to the Move module bytecode file
        #[arg(value_name = "FILE")]
        path: String,
    },
}

pub trait Decoder {
    fn decode(self) -> anyhow::Result<()>;
}

fn read_file(path: &str) -> anyhow::Result<Vec<u8>> {
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join(PathBuf::from(path));
    fs::read(&file_path).with_context(|| format!("Failed to read file: {}", file_path.display()))
}

impl Decoder for InitiaCLI {
    fn decode(self) -> anyhow::Result<()> {
        match &self.cmd {
            InitiaCommand::Decode(cmd) => {
                match &cmd.command {
                    DecodeCommands::Read { path } => {
                        let bytes = read_file(path)?;
                        let result = read_module_info(&bytes)?;
                        let mut json: serde_json::Value = serde_json::from_slice(&result)?;

                        if let Some(address) = json.get_mut("address") {
                            if let serde_json::Value::Array(bytes) = address {
                                let hex = format!("0x{}", bytes.iter()
                                    .filter_map(|b| b.as_u64())
                                    .map(|b| format!("{:02x}", b))
                                    .collect::<String>());
                                *address = serde_json::json!(hex);
                            }
                        }
                        println!("{}", serde_json::to_string_pretty(&json)?);
                    }
                    DecodeCommands::Script { path } => {
                        let bytes = read_file(path)?;
                        let result = decode_script_bytes(bytes)?;
                        let json: serde_json::Value = serde_json::from_slice(&result)?;
                        println!("{}", serde_json::to_string_pretty(&json)?);
                    }
                    DecodeCommands::Module { path } => {
                        let bytes = read_file(path)?;
                        let result = decode_module_bytes(bytes)?;
                        let json: serde_json::Value = serde_json::from_slice(&result)?;
                        println!("{}", serde_json::to_string_pretty(&json)?);
                    }
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}
