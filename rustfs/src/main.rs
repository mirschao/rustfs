#[macro_use]
extern crate rocket;

mod config;
mod handlers;
mod storage;
mod routes;

use rocket::fairing::AdHoc;

#[rocket::launch]
fn rocket() -> _ {
    let config = config::Config::init();

    rocket::build()
        .mount("/", routes::get_routes())
        .attach(AdHoc::on_ignite("Config Setup", |rocket| async {
            println!("RustFS initialized with storage path: {}", config.storage_path);
            Ok(rocket.manage(config))
        }))
}
