mod config;
mod utils;
mod models;
use config::file_config::file_config;
fn main() {
    println!("Hello, world!");
    let config = file_config();
}
