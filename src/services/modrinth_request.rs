use crate::models::models::{SearchResponse, VersionResponse};


pub async fn search_modrinth(client: &Client, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(API_MODRINTH, query);
    
    let res = client.get(&url).send().await?.json::<SearchResponse>().await?;
    
    println!("🔍 Resultados para '{}':", query);
    for hit in res.hits {
        println!(" - {} (ID: {})", hit.title, hit.project_id);
    }
    
    Ok(())
}

pub async fn get_modrinth_download_url(client: &Client, project_slug: &str, game_version: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Ejemplo: /project/{id}/version?game_versions=["1.21.1"]
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
    
    Err("No se encontró ningún archivo compatible".into())
}