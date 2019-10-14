use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[macro_use]
extern crate diesel;

mod db;
mod model;
mod schema;

use crate::model::Note;

fn create_note(note: web::Json<Note>) -> impl Responder {
    println!("capture_note: {:?}", note);
    // let new_note = store_in_db(note.timestamp, &note.title, &note.content);
}

fn get_note(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", req));
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!");
}

fn health() -> impl Responder {
    HttpResponse::Ok().body("{'status': 'ok'}");
}

fn main() {
    let pool =
        db::init_pool("postgres://postgres:docker@localhost/notes").expect("Failed to create pool");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/notes", web::post().to(create_note))
            .route("/notes/{title}", web::get().to(get_note))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
