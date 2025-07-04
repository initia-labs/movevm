use anyhow::Context;
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

use crate::{InitiaCLI, InitiaCommand};

use initia_move_api::handler::{decode_module_bytes, decode_script_bytes, read_module_info};

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
    )]
    Read {
        #[arg(value_name = "PACKAGE_NAME")]
        package_name: String,
        #[arg(value_name = "MODULE_NAME")]
        module_name: String,
        #[clap(
            long = "path",
            short = 'p',
            value_name = "PACKAGE_PATH",
            help = "Path to the package directory"
        )]
        package_path: Option<PathBuf>,
    },

    #[command(
        name = "script",
        about = "Decode Move script bytecode",
        long_about = "Decode Move script bytecode and display its ABI (Application Binary Interface).\n\
        Example: initia-move decode script ./build/package/scripts/my_script.mv"
    )]
    Script {
        #[arg(value_name = "PACKAGE_NAME")]
        package_name: String,
        #[arg(value_name = "SCRIPT_NAME")]
        script_name: String,
        #[clap(
            long = "path",
            short = 'p',
            value_name = "PACKAGE_PATH",
            help = "Path to the package directory"
        )]
        package_path: Option<PathBuf>,
    },

    #[command(
        name = "module",
        about = "Decode Move module bytecode",
        long_about = "Decode Move module bytecode and display its ABI (Application Binary Interface).\n\
        Example: initia-move decode module ./build/package/bytecode_modules/my_module.mv"
    )]
    Module {
        #[arg(value_name = "PACKAGE_NAME")]
        package_name: String,
        #[arg(value_name = "MODULE_NAME")]
        module_name: String,
        #[clap(
            long = "path",
            short = 'p',
            value_name = "PACKAGE_PATH",
            help = "Path to the package directory"
        )]
        package_path: Option<PathBuf>,
    },
}

pub trait Decoder {
    fn decode(self) -> anyhow::Result<()>;
}

fn read_file(package_path: &Option<PathBuf>, path: &str) -> anyhow::Result<Vec<u8>> {
    let current_dir = package_path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));
    let file_path = current_dir.join(PathBuf::from(path));
    fs::read(&file_path).with_context(|| format!("Failed to read file: {}", file_path.display()))
}

impl Decoder for InitiaCLI {
    fn decode(self) -> anyhow::Result<()> {
        match &self.cmd {
            InitiaCommand::Decode(cmd) => {
                match &cmd.command {
                    DecodeCommands::Read {
                        package_name,
                        module_name,
                        package_path,
                    } => {
                        let path =
                            format!("build/{}/bytecode_modules/{}.mv", package_name, module_name);
                        let bytes = read_file(package_path, &path)?;
                        let result = read_module_info(&bytes)?;
                        let mut json: serde_json::Value = serde_json::from_slice(&result)?;

                        if let Some(address) = json.get_mut("address") {
                            if let serde_json::Value::Array(bytes) = address {
                                let hex = format!(
                                    "0x{}",
                                    bytes
                                        .iter()
                                        .filter_map(|b| b.as_u64())
                                        .map(|b| format!("{:02x}", b))
                                        .collect::<String>()
                                );
                                *address = serde_json::json!(hex);
                            }
                        }
                        println!("{}", serde_json::to_string_pretty(&json)?);
                    }
                    DecodeCommands::Script {
                        package_name,
                        script_name,
                        package_path,
                    } => {
                        let path = format!(
                            "build/{}/scripts/bytecode_scripts/{}.mv",
                            package_name, script_name
                        );
                        let bytes = read_file(package_path, &path)?;
                        let result = decode_script_bytes(bytes)?;
                        let json: serde_json::Value = serde_json::from_slice(&result)?;
                        println!("{}", serde_json::to_string_pretty(&json)?);
                    }
                    DecodeCommands::Module {
                        package_name,
                        module_name,
                        package_path,
                    } => {
                        let path =
                            format!("build/{}/bytecode_modules/{}.mv", package_name, module_name);
                        let bytes = read_file(package_path, &path)?;
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
