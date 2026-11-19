use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SearchResponse {
    hits: Vec<ProjectHit>,
}

#[derive(Deserialize, Debug)]
struct ProjectHit {
    project_id: String,
    title: String,
    slug: String,
}

// struct for the response of the version endpoint
#[derive(Deserialize, Debug)]
struct VersionResponse {
    files: Vec<ModFile>,
}

#[derive(Deserialize, Debug)]
struct ModFile {
    url: String,
    filename: String,
    primary: bool,
}