#[cfg(test)]
mod tests {
    use actix_service::Service;
    use actix_web::{http::StatusCode, test, web, App, HttpResponse};

    #[test]
    fn test_add() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn test_health_endpoint() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let mut app = test::init_service(
            App::new().service(web::resource("/health").to(|| HttpResponse::Ok())),
        );

        // Create request object
        let req = test::TestRequest::with_uri("/health").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
