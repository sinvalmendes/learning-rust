use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/{id}/{name}/index.html").to(index))
            .service(web::resource("/index2").to(index2))
            .service(web::resource("/index3").to(index3))
    })
    .bind("0.0.0.0:8080")?
    .run()
}
