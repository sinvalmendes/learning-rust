use crate::model::NewNote;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::db;

pub fn create_note(note: web::Json<NewNote>, pool: web::Data<db::PgPool>) -> impl Responder {
    info!("create_note: {:?}", note);
    let new_note = NewNote {
        title: note.title.clone(),
        content: note.content.clone(),
    };
    let result = db::create_note(new_note, &pool);
    debug!("create_note result: {:?}", result);
    match result {
        Ok(x) => HttpResponse::Created().json(x),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub fn delete_by_title(
    params: web::Path<TitleGetParams>,
    pool: web::Data<db::PgPool>,
) -> impl Responder {
    info!("delete_by_title: {:?}", params);
    let result = db::delete_note_by_title(params.title.clone(), &pool);
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub fn get_all_notes(pool: web::Data<db::PgPool>) -> impl Responder {
    info!("get_all_notes");
    let result = db::get_all_notes(&pool);
    match result {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub fn get_notes_by_title(req: HttpRequest, pool: web::Data<db::PgPool>) -> impl Responder {
    let params = web::Path::<TitleGetParams>::extract(&req).unwrap();

    info!("get_notes_by_title: {:?}", params);
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
