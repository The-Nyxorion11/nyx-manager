use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::utils::utils::get_path;
use crate::models::models::{NyxConfig, Config, LoaderConfig};


//set default values for the NyxConfig struct
impl Default for NyxConfig {
    fn default() -> Self {
        let mut files = HashMap::new();
        files.insert("modName".to_string(), "modName".to_string());

        Self {
            config: Config {
                version: "1.0.0".to_string(),
            },
            loader: LoaderConfig {
                type_: "(plugins, mods)".to_string(),
                name_loader: "example:(paper, purpur, etc...)".to_string(),
                version_minecraft: "example:1.21.1".to_string(),
                version_loader: "example:0.16.5".to_string(),
            },
            files
        }
    }
}

pub fn file_config() -> Result<NyxConfig, Box<dyn std::error::Error>>{
    let path = get_path()?.join("nyx-config.toml");


    // if the file does not exist, create it with default values
    if !path.exists() {
        let toml_str = toml::to_string_pretty(&NyxConfig::default())?;
        fs::write(&path, toml_str)?;
    }

    // read the file
    let content = fs::read_to_string(&path)?;
    let nyx_config: NyxConfig = toml::from_str(&content)?;
    println!("nyx-config: umazing");

    //umazing
    Ok((nyx_config))
}
