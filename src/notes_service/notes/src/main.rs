use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
struct Note {
    title: String,
    content: String,
}

fn capture_note(note: web::Json<Note>) -> impl Responder {
    // let new_note = store_in_db(note.timestamp, &note.title, &note.content);
    // format!("got note {}", new_note.id.unwrap())
    println!("capture_note: {:?}", note);
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!");
}

fn health() -> impl Responder {
    HttpResponse::Ok().body("{'status': 'ok'}");
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/notes", web::post().to(capture_note))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
