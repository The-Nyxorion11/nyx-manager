mod services;
mod config;
mod utils;
mod models;

use services::modrinth_request::search_modrinth;
use config::file_config::file_config;

use crate::models::models::NyxConfig;

#[tokio::main]
async fn main() {

    let config = file_config().expect("error load config");
    let client = reqwest::Client::builder()
    .user_agent("nyx-gestor/0.1.0 (github.com/The-Nyxorion11/nyx-gestor)")
    .build()
    .expect("Failed to build reqwest client");
    let result = search_modrinth(&client, &config).await;

    
}
