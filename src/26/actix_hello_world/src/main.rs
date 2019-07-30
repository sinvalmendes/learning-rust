use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// This struct represents state
struct AppState {
    app_name: String,
}

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn index2(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

fn index3(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    HttpResponse::Ok().body(format!("Hello again {}!", app_name))
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("world"),
            })
            .service(web::resource("/{id}/{name}/index.html").to(index))
            .service(web::resource("/index2").to(index2))
            .service(web::resource("/index3").to(index3))
    })
    .bind("0.0.0.0:8080")?
    .run()
}

// Implement shared state with set (url based) and get
// Change set to receive JSON post
