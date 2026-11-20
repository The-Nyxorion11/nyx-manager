use crate::models::models::{ModrinthDependency, NyxConfig, SearchResponse, VersionResponse};
use crate::utils::utils::get_path;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;


#[derive(Deserialize)]
struct ProjectInfo {
    slug: String,
}

async fn get_project_slug(
    client: &Client,
    id_or_slug: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.modrinth.com/v2/project/{}", id_or_slug);
    let info: ProjectInfo = client.get(&url).send().await?.json().await?;
    Ok(info.slug)
}

pub struct ResolvedDownload {
    pub url: String,
    pub dependencies: Vec<ModrinthDependency>,
}

pub async fn search_modrinth(
    client: &Client,
    nyx_config: &NyxConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let type_loader = &nyx_config.loader.type_;
    let game_version = &nyx_config.loader.version_minecraft;
    let output_dir = get_path()?.join(type_loader);

    if type_loader != "mods" && type_loader != "plugins" {
        println!("unknown type: {}", type_loader);
        return Ok(());
    }

    let mut downloaded: HashSet<String> = HashSet::new();

    for (_key, value) in &nyx_config.files {
        let query = value.trim();
        if query.is_empty() {
            continue;
        }

        println!("Searching for: {}", query);
        let search_url = format!("https://api.modrinth.com/v2/search?query={}", query);

        let res: SearchResponse = client
            .get(&search_url)
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;

        let Some(hit) = res.hits.first() else {
            println!("No results found for: {}", query);
            continue;
        };

        if let Err(e) = download_with_deps(
            client,
            &hit.slug,
            game_version,
            type_loader,
            nyx_config,
            &output_dir,
            &mut downloaded,
        )
        .await
        {
            println!("Skipping {} ({}): {}", hit.slug, game_version, e);
        }
    }

    Ok(())
}

fn download_with_deps<'a>(
    client: &'a Client,
    project_id_or_slug: &'a str,
    game_version: &'a str,
    type_: &'a str,
    nyx_config: &'a NyxConfig,
    output_dir: &'a Path,
    downloaded: &'a mut HashSet<String>,
) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + 'a>> {
    Box::pin(async move {
        if !downloaded.insert(project_id_or_slug.to_string()) {
            return Ok(());
        }

        let resolved =
            get_modrinth_download_url(client, project_id_or_slug, game_version, type_, nyx_config)
                .await?;

        let slug = get_project_slug(client, project_id_or_slug)
            .await
            .unwrap_or_else(|_| project_id_or_slug.to_string());

        let filename = format!("{}.jar", slug);
        download_jar(client, &resolved.url, output_dir, &filename).await?;

        for dep in resolved.dependencies {
            if dep.dependency_type == "required" {
                if let Some(dep_project_id) = dep.project_id {
                    if let Err(e) = download_with_deps(
                        client,
                        &dep_project_id,
                        game_version,
                        type_,
                        nyx_config,
                        output_dir,
                        downloaded,
                    )
                    .await
                    {
                        println!("Warning: could not download dependency {}: {}", dep_project_id, e);
                    }
                }
            }
        }

        Ok(())
    })
}

pub async fn get_modrinth_download_url(
    client: &Client,
    project_slug: &str,
    game_version: &str,
    type_: &str,
    nyx_config: &NyxConfig,
) -> Result<ResolvedDownload, Box<dyn std::error::Error>> {
    let loader_name = &nyx_config.loader.name_loader;

    println!(
        "Fetching download URL for project: {} with game version: {}",
        project_slug, game_version
    );
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version?loaders=[\"{}\"]&game_versions=[\"{}\"]",
        project_slug, loader_name, game_version
    );
    println!("Requesting URL: {}", url);

    let versions: Vec<VersionResponse> = client.get(&url).send().await?.json().await?;

    if let Some(latest_version) = versions.first() {
        if let Some(file) = latest_version
            .files
            .iter()
            .find(|f| f.primary)
            .or(latest_version.files.first())
        {
            return Ok(ResolvedDownload {
                url: file.url.clone(),
                dependencies: latest_version.dependencies.clone(),
            });
        }
    }

    let _ = type_;
    Err("error file not found".into())
}

pub async fn download_jar(
    client: &reqwest::Client,
    download_url: &str,
    output_dir: &Path,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(output_dir)?;

    let file_path = output_dir.join(filename);
    println!("downloading... {}", filename);

    let bytes = client.get(download_url).send().await?.bytes().await?;
    std::fs::write(&file_path, bytes)?;

    println!("save in: {:?}", file_path);
    Ok(())
}