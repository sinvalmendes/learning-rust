
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
        assert_eq!(response.body_string(), Some("{\"result\":\"Not found\"}".into()));
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
        assert_eq!(response_get.status(), Status::Ok);
        assert_eq!(response_get.body_string(), Some("{\"test_value\":\"blablabla\"}".into()));

    }
}
