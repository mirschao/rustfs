use std::fs;
use std::path::PathBuf;
use rocket::serde::json::Json;
use rocket::Data;

pub struct Storage {
    pub base_path: String,
}

impl Storage {
    pub fn new(base_path: String) -> Self {
        Storage { base_path }
    }

    pub fn save_file(&self, filename: &str, data: Data) -> Result<(), String> {
        let filepath = PathBuf::from(&self.base_path).join(filename);
        let mut file = fs::File::create(&filepath).map_err(|e| e.to_string())?;
        data.stream_to(&mut file).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn read_file(&self, filename: &str) -> Result<Vec<u8>, String> {
        let filepath = PathBuf::from(&self.base_path).join(filename);
        fs::read(filepath).map_err(|e| e.to_string())
    }
}
