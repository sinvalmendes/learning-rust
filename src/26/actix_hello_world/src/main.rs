use actix_web::{error, web, App, FromRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
struct Info {
    app_name: String,
}

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
    format!("Incremented counter: {}", counter) // <- response with count
}

fn _index2(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    format!("Counter: {}", counter)
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

fn index_json(info: web::Json<Info>) -> impl Responder {
    format!("Hello (json) {}!", info.app_name)
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
            .service(web::resource("/increment/counter").to(_index))
            .service(web::resource("/read/counter").to(_index2))
            .service(web::resource("/{id}/{name}/index.html").to(index))
            .service(web::resource("/index2").to(index2))
            .service(web::resource("/index3").to(index3))
            .service(
                web::resource("/")
                    .data(
                        // change json extractor configuration
                        web::Json::<Info>::configure(|cfg| {
                            cfg.limit(4096).error_handler(|err, _req| {
                                // create custom error response
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                )
                                .into()
                            })
                        }),
                    )
                    .route(web::post().to(index_json)),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
}


// fn main() {
//     println!("Hello world!")
// }