#[cfg(test)]
mod tests {
    use crate::api::{create_note, get_all_notes, health, index};
    use crate::db::*;
    use actix_service::Service;
    use actix_web::middleware::Logger;
    use actix_web::{http::StatusCode, test, web, App};
    use bytes::Bytes;
    use dotenv::dotenv;
    use std::env;

    #[test]
    fn test_add() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn test_index_endpoint() {
        let mut app = test::init_service(App::new().service(web::resource("/").to(index)));

        // Create request object
        let req = test::TestRequest::with_uri("/").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let result = test::read_body(resp);
        assert_eq!(result, Bytes::from_static(b"Hello world!"));
    }

    #[test]
    fn test_health_endpoint() {
        let mut app = test::init_service(App::new().service(web::resource("/health").to(health)));

        // Create request object
        let req = test::TestRequest::with_uri("/health").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let result = test::read_body(resp);
        assert_eq!(result, Bytes::from_static(b"{\"status\": \"ok\"}"));
    }

    #[test]
    fn test_create_notes_endpoint() {
        dotenv().ok();

        env::set_var("RUST_LOG", "debug,actix_todo=debug,actix_web=info");
        env_logger::init();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = init_pool(&database_url).expect("Failed to create pool");

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .wrap(Logger::default())
                .service(web::resource("/notes").to(create_note)),
        );

        // Create request object
        let req = test::TestRequest::post()
            .set_payload(Bytes::from_static(
                b"{\"title\": \"abc\",\"content\": \"bla\"}",
            ))
            .header("Content-Type", "application/json")
            .uri("/notes")
            .to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_get_all_notes_endpoint() {
        dotenv().ok();

        env::set_var("RUST_LOG", "debug,actix_todo=debug,actix_web=info");

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = init_pool(&database_url).expect("Failed to create pool");

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .wrap(Logger::default())
                .service(web::resource("/notes").to(get_all_notes)),
        );

        // Create request object
        let req = test::TestRequest::with_uri("/notes").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
