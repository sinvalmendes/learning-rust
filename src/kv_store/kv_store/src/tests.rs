
#[cfg(test)]
mod tests {
    use rocket::local::Client;
    use crate::get_rocket;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("Hello tests");

        let rocket = get_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/api");
        let response = req.dispatch();
        println!("{:?}", response);
    }
}
