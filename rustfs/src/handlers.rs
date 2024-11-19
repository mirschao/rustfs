use rocket::serde::json::Json;
use rocket::{Data, State};
use crate::storage::Storage;
use crate::config::Config;

#[post("/upload/<filename>", data = "<data>")]
pub async fn upload_file(
    filename: String,
    data: Data<'_>,
    config: &State<Config>,
) -> Json<String> {
    let storage = Storage::new(config.storage_path.clone());
    match storage.save_file(&filename, data).await {
        Ok(_) => Json(format!("File '{}' uploaded successfully!", filename)),
        Err(e) => Json(format!("Failed to upload file '{}': {}", filename, e)),
    }
}

#[get("/download/<filename>")]
pub async fn download_file(
    filename: String,
    config: &State<Config>,
) -> Option<Vec<u8>> {
    let storage = Storage::new(config.storage_path.clone());
    match storage.read_file(&filename) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}
