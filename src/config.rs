use std::io::Write;

use crate::Library;
use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct CBuildConfig {
    pub input: String,
    pub output: String,
    pub libs: Vec<Library>,
    pub std: Vec<String>
}

impl CBuildConfig {
    pub fn get_compile_command(&self) -> String {
        let input = &self.input;
        let output = &self.output;
    
        let linking: Vec<String> = self.libs.iter().map(|x| x.get_link_string()).collect();
        let linker_lib_strings = linking.join(" ").to_string();
        let linker_std_strings = self.std
                                    .iter()
                                    .map(|x| format!("-{}", x))
                                    .collect::<Vec<String>>()
                                    .join(" ").to_string();
    
        format!("clang++ {input} -o {output} {linker_lib_strings} {linker_std_strings}")
    }
}

impl Default for CBuildConfig {
    fn default() -> Self {
        return CBuildConfig { 
            input: "main.cpp".to_string(),
            output: "main".to_string(),
            libs: vec![
                Library{
                    name: "SDL2".to_string(),
                    git_url: "https://github.com/libsdl-org/SDL.git".to_string(),
                    build_command: Some("cmake -S . -B build && cmake --build build && cmake --install build".to_string())
                }
            ], 
            std: vec!["lstdc++".to_string(), "lm".to_string()] 
        }
    }

    
}

pub fn read_config_file() -> Result<CBuildConfig> {
    let text = std::fs::read_to_string("./c_build_config.json")?;
    let config: CBuildConfig = serde_json::from_str(&text)?;
    return Ok(config);
}

pub fn create_config_file() -> Result<()> {
    let default_config = CBuildConfig::default();
    let data_string: Vec<u8> = serde_json::to_vec_pretty(&default_config)?;
    let mut file = std::fs::File::create("./c_build_config.json")?;
    file.write_all(&data_string)?;
    Ok(())
}