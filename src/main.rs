mod libraries;
mod commands;
mod config;
mod actions;

use anyhow::Result;
use std::process::Command;
use anyhow::bail;
use config::CBuildConfig;
use libraries::*;
use commands::Commands;
use actions::*;
use nix::unistd::{geteuid, Uid};





fn main() -> Result<()> {
    let command = commands::get_command();

    match command {
        Commands::Init => {
            let res = init();
            match res {
                Ok(_) => {println!("Init succesful")},
                Err(err) => {println!("An error has ocurred: {}", err.to_string())}
            }
        },
        Commands::Compile => {
            let config = match config::read_config_file() {
                Ok(config) => config,
                Err(_) => {bail!("Failed to read config file")}
            };
            let _result = compile(config)?;
            println!("compiler exit succesful");

        },
        Commands::Install => {
            let euid = geteuid();
            if !(euid == Uid::from_raw(0)) {
                bail!("This command must be run as sudo");
            }

            let config = match config::read_config_file() {
                Ok(config) => config,
                Err(_) => {bail!("Failed to read config file")}
            };
            let _res = install(config)?;
            println!("All libraries installed succesfully!")
        },
        Commands::Run => {
            let config = match config::read_config_file() {
                Ok(config) => config,
                Err(_) => {bail!("Failed to read config file")}
            };
            let _result = compile(config.clone())?;

            let _status = Command::new("sh")
                .arg("-c")
                .arg(format!("./{}", config.output))
                .status();

        }
        
        _ => panic!("not implemented yet")
    }
    Ok(())
}
