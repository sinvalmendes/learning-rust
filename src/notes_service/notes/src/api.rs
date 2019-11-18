use crate::model::NewNote;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::db;

pub fn create_note(note: web::Json<NewNote>, pool: web::Data<db::PgPool>) -> impl Responder {
    let new_note = NewNote {
        title: note.title.clone(),
        content: note.content.clone(),
    };
    match db::create_note(new_note, &pool) {
        Ok(x) => HttpResponse::Created().json(x),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub fn delete_by_title(req: HttpRequest, pool: web::Data<db::PgPool>) -> impl Responder {
    let params = web::Path::<TitleGetParams>::extract(&req).unwrap();

    let result = db::delete_note_by_title(params.title.clone(), &pool);
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub fn get_all_notes(pool: web::Data<db::PgPool>) -> impl Responder {
    match db::get_all_notes(&pool) {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub fn get_notes_by_title(req: HttpRequest, pool: web::Data<db::PgPool>) -> impl Responder {
    let params = web::Path::<TitleGetParams>::extract(&req).unwrap();

    let result = db::select_by_title(params.title.clone(), &pool);
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

#[derive(Debug, Deserialize)]
pub struct TitleGetParams {
    title: String,
}

pub fn index() -> impl Responder {
    return HttpResponse::Ok()
        .content_type("plain/text")
        .body("Hello world!");
}

pub fn health() -> impl Responder {
    return HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"status\": \"ok\"}");
}
