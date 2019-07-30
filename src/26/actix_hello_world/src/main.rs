use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// This struct represents state
struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

fn _index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
    format!("Request number: {}", counter) // <- response with count
}

fn _index2(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    format!("Request number: {}", counter)
}

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn index2(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

fn index3(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello again {}!", app_name))
}

fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("world"),
            })
            .register_data(counter.clone()) // <- register the created data
            .service(web::resource("/counter").to(_index))
            .service(web::resource("/read/counter").to(_index2))
            .service(web::resource("/{id}/{name}/index.html").to(index))
            .service(web::resource("/index2").to(index2))
            .service(web::resource("/index3").to(index3))
    })
    .bind("0.0.0.0:8080")?
    .run()
}

// Implement shared state with set (url based) and get
// Change set to receive JSON post
