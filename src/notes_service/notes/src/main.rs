use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
