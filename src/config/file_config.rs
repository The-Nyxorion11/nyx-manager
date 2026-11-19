use crate::utils::utils::get_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
struct NyxConfig {
    config: Config,       // [config]
    loader: LoaderConfig, // [loader]
    #[serde(default)]
    files: HashMap<String, String>,        // [files]
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoaderConfig {
    #[serde(rename = "type")] 
    type_: String,
    name: String,
    version_minecraft: String,
    version_loader: String,
}

//set default values for the NyxConfig struct
impl Default for NyxConfig {
    fn default() -> Self {
        let mut files = HashMap::new();
        files.insert("modExample".to_string(), "modExample".to_string());

        Self {
            config: Config {
                version: "1.0.0".to_string(),
            },
            loader: LoaderConfig {
                type_: "paper".to_string(),
                name: "NyxInstance".to_string(),
                version_minecraft: "1.21.1".to_string(),
                version_loader: "0.16.5".to_string(),
            },
            files
        }
    }
}

fn file_config() -> Result<NyxConfig, Box<dyn std::error::Error>>{
    let path = Path::new(get_path()?.join("nyx_config.toml"));

    // if the file does not exist, create it with default values
    if !path.exists() {
        let toml_str = toml::to_string_pretty(&NyxConfig::default())?;
        fs::write(path, toml_str)?;
    }

    // read the file
    let content = fs::read_to_string(path)?;
    let nyx_config: NyxConfig = toml::from_str(&content)?;
    println!("{:#?}", nyx_config);

    //umazing
    Ok((nyx_config))
}
