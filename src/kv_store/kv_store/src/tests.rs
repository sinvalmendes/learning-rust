
#[cfg(test)]
mod tests {
    use rocket::local::Client;
    use crate::get_rocket;
    use rocket::http::Status;
    use rocket::http::ContentType;

    #[test]
    fn test_get_api() {
        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/api");
        let mut response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{}".into()));
    }

    #[test]
    fn test_get_value_not_found() {
        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/api/kv/test_value");
        let mut response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{\"error\":\"Not found\"}".into()));

        let custom_headers = response.headers();
        assert_eq!(custom_headers.get_one("Content-Type"), Some("application/json"));
    }

    #[test]
    fn test_get_value() {
        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");

        let req = client.put("/api/kv/test_value")
            .body("{\"value\":\"blablabla\"}")
            .header(ContentType::JSON);

        let response_put = req.dispatch();
        assert_eq!(response_put.status(), Status::Ok);

        let req = client.get("/api/kv/test_value");
        let mut response_get = req.dispatch();

        assert_eq!(response_get.body_string(), Some("{\"test_value\":\"blablabla\"}".into()));
        assert_eq!(response_get.status(), Status::Ok);

        let custom_headers = response_get.headers();
        assert_eq!(custom_headers.get_one("Content-Type"), Some("application/json"));
    }

    #[test]
    fn test_put_value_invalid_format_causes_bad_request() {
        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");

        let req = client.put("/api/kv/test_value")
            .body("{\"value\":\"\"")
            .header(ContentType::JSON);
        let response_put = req.dispatch();
        assert_eq!(response_put.status(), Status::BadRequest);

        let custom_headers = response_put.headers();
        assert_eq!(custom_headers.get_one("Content-Type"), Some("text/html; charset=utf-8"));
    }

    #[test]
    fn test_put_value_invalid_key_format_causes_bad_request() {
        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");

        let req = client.put("/api/kv/test_value")
            .body("{\"\":\"value without a key\"")
            .header(ContentType::JSON);
        let response_put = req.dispatch();
        assert_eq!(response_put.status(), Status::BadRequest);

        let custom_headers = response_put.headers();
        assert_eq!(custom_headers.get_one("Content-Type"), Some("text/html; charset=utf-8"));
    }

    #[test]
    fn test_get_value_invalid_content_type_causes_not_found() {
        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");

        let req = client.put("/api/kv/test_value")
            .body("{\"value\":\"blablabla\"}")
            .header(ContentType::HTML);
        let response_put = req.dispatch();
        assert_eq!(response_put.status(), Status::NotFound);

        let custom_headers = response_put.headers();
        assert_eq!(custom_headers.get_one("Content-Type"), Some("text/html; charset=utf-8"));
    }

    #[test]
    fn test_get_value_without_content_type_causes_not_found() {
        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");

        let req = client.put("/api/kv/test_value")
            .body("{\"value\":\"blablabla\"}");
        let response_put = req.dispatch();
        assert_eq!(response_put.status(), Status::NotFound);

        let custom_headers = response_put.headers();
        assert_eq!(custom_headers.get_one("Content-Type"), Some("text/html; charset=utf-8"));
    }
}
