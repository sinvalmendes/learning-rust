use crate::model::NewNote;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;

pub fn create_note(note: web::Json<NewNote>, pool: web::Data<db::PgPool>) -> impl Responder {
    println!("create_note: {:?}", note);
    let new_note = NewNote {
        title: note.title.clone(),
        content: note.content.clone(),
    };
    let result = db::create_note(new_note, &pool);
    println!("create_note result: {:?}", result)
}

pub fn get_all_notes(req: HttpRequest, pool: web::Data<db::PgPool>) -> impl Responder {
    let result = db::get_all_notes(&pool);
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub fn index() -> impl Responder {
    println!("index");
    HttpResponse::Ok().body("Hello world!");
}

pub fn health() -> impl Responder {
    println!("health");
    HttpResponse::Ok().body("{'status': 'ok'}");
}
