use anyhow::Context;
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

use crate::{InitiaCLI, InitiaCommand};

use initia_move_api::handler::{decode_module_bytes, decode_script_bytes};

#[derive(Parser)]
#[command(
    name = "decode",
    about = "Decode Move modules and scripts",
    long_about = "Decode Move modules and prints the result in JSON format"
)]
pub struct Decode {
    #[command(subcommand)]
    pub command: DecodeCommands,
}

#[derive(Subcommand)]
pub enum DecodeCommands {
    #[command(
        name = "script",
        about = "Decode Move script bytecode",
        long_about = "Decode Move script bytecode and display its ABI (Application Binary Interface).\n\
        Example: initia-move-cli decode script ./build/package/scripts/my_script.mv"
    )]
    Script {
        #[arg(value_name = "PATH")]
        path: String,
    },

    #[command(
        name = "module",
        about = "Decode Move module bytecode",
        long_about = "Decode Move module bytecode and display its ABI (Application Binary Interface).\n\
        Example: initia-move-cli decode module ./build/package/bytecode_modules/my_module.mv"
    )]
    Module {
        #[arg(value_name = "PATH")]
        path: String,
    },
}

pub trait Decoder {
    fn decode(self) -> anyhow::Result<()>;
}

fn read_file(path: &str) -> anyhow::Result<Vec<u8>> {
    let file_path = PathBuf::from(path);
    fs::read(&file_path).with_context(|| format!("Failed to read file: {}", file_path.display()))
}

impl Decoder for InitiaCLI {
    fn decode(self) -> anyhow::Result<()> {
        match &self.cmd {
            InitiaCommand::Decode(cmd) => {
                match &cmd.command {
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
