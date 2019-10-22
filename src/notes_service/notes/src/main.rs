use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::env;

#[cfg(test)]
mod tests;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

mod api;
mod db;
mod model;
mod schema;

fn main() {
    env::set_var("RUST_LOG", "info,actix_todo=debug,actix_web=info");
    env_logger::init();
    info!("started!");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(api::index))
            .route("/health", web::get().to(api::health))
            .route("/api/notes", web::get().to(api::get_all_notes))
            .route("/api/notes/{title}", web::get().to(api::get_notes_by_title))
            .route("/api/notes", web::post().to(api::create_note))
    })
    .bind("0.0.0.0:8000")
    .unwrap()
    .run()
    .unwrap();
}
