use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
    MODRINTH API STRUCTS
*/
#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    hits: Vec<ProjectHit>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectHit {
    project_id: String,
    title: String,
    slug: String,
}

// struct for the response of the version endpoint
#[derive(Deserialize, Debug)]
pub struct VersionResponse {
    files: Vec<ModFile>,
}

#[derive(Deserialize, Debug)]
pub struct ModFile {
    url: String,
    filename: String,
    primary: bool,
}


/*
    NYX-CONFIG STRUCTS
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct NyxConfig {
    pub config: Config,       // [config]
    pub loader: LoaderConfig, // [loader]
    #[serde(default)]
    pub files: HashMap<String, String>,        // [files]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoaderConfig {
    #[serde(rename = "type_loader")] 
    pub type_: String,
    pub name_loader: String,
    pub version_minecraft: String,
    pub version_loader: String,
}
