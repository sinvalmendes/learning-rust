use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn main() {
    println!("Hello, world!");
}
