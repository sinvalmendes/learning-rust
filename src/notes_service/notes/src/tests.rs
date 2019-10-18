#[cfg(test)]
mod tests {
    use crate::api::{health, index};

    use actix_service::Service;
    use actix_web::{http::StatusCode, test, web, App};
    use bytes::Bytes;

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
        // let body = resp.take_body();
        // println!("{:?}", body.as_ref().unwrap());
        let result = test::read_body(resp);
        assert_eq!(result, Bytes::from_static(b"{\"status\": \"ok\"}"));
    }
}
