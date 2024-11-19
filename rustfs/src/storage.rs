use rocket::data::{ByteUnit, Data};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct Storage {
    pub base_path: String,
}

impl Storage {
    pub fn new(base_path: String) -> Self {
        Storage { base_path }
    }

    pub async fn save_file(&self, filename: &str, data: Data<'_>) -> Result<(), String> {
        let filepath = PathBuf::from(&self.base_path).join(filename);
        let mut file = File::create(&filepath).map_err(|e| e.to_string())?;

        let mut stream = data.open(ByteUnit::default());
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| e.to_string())?;
            file.write_all(&chunk).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn read_file(&self, filename: &str) -> Result<Vec<u8>, String> {
        let filepath = PathBuf::from(&self.base_path).join(filename);
        fs::read(filepath).map_err(|e| e.to_string())
    }
}
