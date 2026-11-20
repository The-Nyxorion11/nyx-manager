use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
    MODRINTH API STRUCTS
*/
#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub hits: Vec<ProjectHit>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectHit {
    pub project_id: String,
    pub title: String,
    pub slug: String,
}

// struct for the response of the version endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct VersionFile {
    pub url: String,
    pub filename: String,
    #[serde(default)]
    pub primary: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModrinthDependency {
    pub project_id: Option<String>,
    pub version_id: Option<String>,
    pub dependency_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VersionResponse {
    pub files: Vec<VersionFile>,
    #[serde(default)]
    pub dependencies: Vec<ModrinthDependency>,
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
