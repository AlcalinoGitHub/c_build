use std::process::Command;
use anyhow::{Result, bail};
use crate::CBuildConfig;
use crate::config;

pub fn compile(config: CBuildConfig) -> Result<()> {
    let command = config.get_compile_command();

    let status = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .status();

    match status.unwrap().success() {
        true => {
            return Ok(())
        },
        false => {
            println!("compiler exit failure");
            bail!("COMPILATION FAILED")
        }
    }
}

pub fn init() -> Result<()> {
    std::fs::create_dir("./c_build_modules")?;
    config::create_config_file()?;
    Ok(())
}

pub fn install(config: CBuildConfig) -> Result<()> {
    let libs = config.libs;

    for library in libs {
        if library.is_installed()? {
            println!("library {} is already installed. skipping", library.name);
            continue;
        }
        let command = library.get_build_commands().join(" && ");

        let status = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .status();

        let error = status.as_ref().unwrap().to_string();
        let lib_name = library.name;

        if !status.unwrap().success() {
            bail!("Library {lib_name} failed to install: {error}")
        } else {
            println!("Library {lib_name} installed succesfully")
        }
    }

    Ok(())
}