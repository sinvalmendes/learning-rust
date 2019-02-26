
#[cfg(test)]
mod tests {
    use rocket::local::Client;
    use crate::get_rocket;
    use rocket::http::Status;

    #[test]
    fn it_works() {
        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/api");
        let mut response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{}".into()));
    }
}
