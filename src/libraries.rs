use std::process::Command;

use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Library {
    pub name: String,
    pub git_url: String,
    pub build_command: Option<String>
}


impl Library {
    pub fn get_link_string(&self) -> String {
        let name = &self.name;
        return format!("-l{name} -L./c_build_modules/{name}")
    }

    pub fn get_build_commands(&self) -> Vec<String> {
        let clone = format!("git clone {} ./c_build_modules/{}", self.git_url, self.name);
        let cd = format!("cd c_build_modules/{}", self.name);
        let build = match &self.build_command {
            Some(command) => "sudo ".to_string() + &command.to_string(),
            None => r#"echo "no build command detected" "#.to_string()
        };

        return vec![clone, cd, build];
    }

    pub fn is_installed(&self) -> Result<bool> {
        let folders = std::fs::read_dir("./c_build_modules")?;
    
        for i in folders {
            if i.unwrap().file_name().to_string_lossy().to_string() == self.name {
                return Ok(true)
            }
        }
        Ok(false)
    }

}