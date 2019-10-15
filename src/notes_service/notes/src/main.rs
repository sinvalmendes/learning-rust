use crate::model::NewNote;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

mod db;
mod model;
mod schema;

fn create_note(note: web::Json<NewNote>, pool: web::Data<db::PgPool>) -> impl Responder {
    println!("create_note: {:?}", note);
    let new_note = NewNote {
        title: note.title.clone(),
        content: note.content.clone(),
    };
    let result = db::create_note(new_note, &pool);
    println!("create_note result: {:?}", result)
}

fn get_all_notes(req: HttpRequest, pool: web::Data<db::PgPool>) -> impl Responder {
    println!("get_all_notes: {:?}", req);
    let result = db::get_all_notes(&pool);
    println!("get_all_notes result {:?}", result);
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

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/notes", web::get().to(get_all_notes))
            .route("/notes", web::post().to(create_note))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
