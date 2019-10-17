#[cfg(test)]
mod tests {
    use crate::api::health;
    use crate::api::index;

    use actix_service::Service;
    use actix_web::{http::StatusCode, test, web, App};

    #[test]
    fn test_add() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn test_health_endpoint() {
        let mut app = test::init_service(App::new().service(web::resource("/health").to(index)));

        // Create request object
        let req = test::TestRequest::with_uri("/health").to_request();

        // Execute application
        let mut resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.take_body();
        println!("{:?}", body.as_ref().unwrap());
    }
}
