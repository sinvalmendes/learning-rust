use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

mod db;
mod model;
mod schema;

use crate::model::Note;

fn create_note(note: web::Json<Note>) -> impl Responder {
    println!("capture_note: {:?}", note);
    // let new_note = store_in_db(note.timestamp, &note.title, &note.content);
}

fn get_note(req: HttpRequest) -> impl Responder {
    println!("get_note: {:?}", req);
    HttpResponse::Ok().body(format!("{:?}", req));
}

fn index() -> impl Responder {
    println!("index");
    HttpResponse::Ok().body("Hello world!");
}

fn health() -> impl Responder {
    println!("health");
    HttpResponse::Ok().body("{'status': 'ok'}");
}

fn main() {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/notes/{title}", web::get().to(get_note))
            .route("/notes", web::post().to(create_note))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
