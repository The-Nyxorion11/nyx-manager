use crate::models::models::{SearchResponse, VersionResponse};
use std::path::Path;

pub async fn search_modrinth(client: &Client, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for mods with query: {}", query);
    let url = format!(API_MODRINTH, query);
    
    let res = client.get(&url).send().await?.json::<SearchResponse>().await?;
    
    println!("result: {}", query);
    for hit in res.hits {
        println!(" - {} (ID: {})", hit.title, hit.project_id);
    }
    
    Ok(())
}

pub async fn get_modrinth_download_url(client: &Client, project_slug: &str, game_version: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Fetching download URL for project: {} with game version: {}", project_slug, game_version);
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]",
        project_slug, game_version
    );

    let versions: Vec<VersionResponse> = client.get(&url).send().await?.json().await?;
    
    if let Some(latest_version) = versions.first() {
        if let Some(file) = latest_version.files.iter().find(|f| f.primary).or(latest_version.files.first()) {
            return Ok(file.url.clone());
        }
    }
    
    Err("error file not found".into())
}


pub async fn download_jar(client: &reqwest::Client, download_url: &str, output_dir: &Path, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(output_dir)?;
    
    let file_path = output_dir.join(filename);
    println!("downloading... {}", filename);

    // get request to download the file
    let bytes = client.get(download_url).send().await?.bytes().await?;

    // Guardar los bytes en el disco duro
    std::fs::write(&file_path, bytes)?;
    
    println!("save in: {:?}", file_path);
    Ok(())
}