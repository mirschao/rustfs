use rocket::Route;

use crate::handlers::*;

pub fn get_routes() -> Vec<Route> {
    routes![upload_file, download_file]
}
