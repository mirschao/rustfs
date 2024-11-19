use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_path: String,
}

impl Config {
    pub fn init() -> Self {
        let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "./storage".to_string());
        std::fs::create_dir_all(&storage_path).expect("Failed to create storage directory");
        Config { storage_path }
    }
}
