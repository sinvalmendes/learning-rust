#[cfg(test)]
mod tests {
    use crate::api::{
        create_note, delete_by_title, get_all_notes, get_notes_by_title, health, index,
    };
    use crate::db::*;
    use crate::model::NewNote;
    use actix_service::Service;
    use actix_web::middleware::Logger;
    use actix_web::{http::StatusCode, test, web, App};
    use bytes::Bytes;
    use dotenv::dotenv;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::env;
    use std::iter;
    use std::sync::Once;

    static INIT: Once = Once::new();
    fn setup() {
        dotenv().ok();

        env::set_var("RUST_LOG", "debug,actix_todo=debug,actix_web=info");
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> (),
    {
        setup();

        test();
    }

    #[test]
    fn test_add() {
        run_test(|| {
            assert_eq!(4, 2 + 2);
        })
    }

    #[test]
    fn test_index_endpoint() {
        run_test(|| {
            let mut app = test::init_service(App::new().service(web::resource("/").to(index)));

            let req = test::TestRequest::with_uri("/").to_request();
            let resp = test::block_on(app.call(req)).unwrap();

            assert_eq!(resp.status(), StatusCode::OK);
            let result = test::read_body(resp);
            assert_eq!(result, Bytes::from_static(b"Hello world!"));
        })
    }

    #[test]
    fn test_health_endpoint() {
        run_test(|| {
            let mut app =
                test::init_service(App::new().service(web::resource("/health").to(health)));

            let req = test::TestRequest::with_uri("/health").to_request();
            let resp = test::block_on(app.call(req)).unwrap();

            assert_eq!(resp.status(), StatusCode::OK);
            let result = test::read_body(resp);
            assert_eq!(result, Bytes::from_static(b"{\"status\": \"ok\"}"));
        })
    }

    #[test]
    fn test_create_notes_endpoint() {
        run_test(|| {
            create_note_post_request("note_name", "bla");
            create_note_post_request("note_name", "");
            create_note_post_request("", "");
            create_note_post_request(
                "looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong_note_name",
                "",
            );
            create_note_post_request(
                "note_name",
                "loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong content",
            );
        })
    }

    #[test]
    fn test_get_all_notes_endpoint() {
        run_test(|| {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = init_pool(&database_url).expect("Failed to create pool");

            let mut app = test::init_service(
                App::new()
                    .data(pool.clone())
                    .wrap(Logger::default())
                    .service(web::resource("/notes").to(get_all_notes)),
            );

            let req = test::TestRequest::with_uri("/notes").to_request();
            let resp = test::block_on(app.call(req)).unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        })
    }

    fn create_note_post_request(note_name: &str, note_content: &str) -> NewNote {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = init_pool(&database_url).expect("Failed to create pool");

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .wrap(Logger::default())
                .service(web::resource("/notes").to(create_note)),
        );

        let payload: String = format!(
            "{{\"title\": \"{}\",\"content\": \"{}\"}}",
            note_name, note_content
        );

        let req = test::TestRequest::post()
            .set_payload(payload)
            .header("Content-Type", "application/json")
            .uri("/notes")
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);

        let note_created = NewNote {
            title: note_name.to_string(),
            content: String::from(note_content),
        };
        return note_created;
    }

    #[test]
    fn test_get_note_by_title_endpoint() {
        run_test(|| {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = init_pool(&database_url).expect("Failed to create pool");
            let note_name: String = iter::repeat(())
                .map(|()| thread_rng().sample(Alphanumeric))
                .take(7)
                .collect();
            let note_created = create_note_post_request(&note_name, "bla");

            let mut app = test::init_service(
                App::new()
                    .data(pool.clone())
                    .wrap(Logger::default())
                    .service(web::resource("/api/notes/{title}").to(get_notes_by_title)),
            );
            let uri = format!("/api/notes/{}", note_created.title);
            let req = test::TestRequest::with_uri(uri.as_str()).to_request();
            let result: Vec<NewNote> = test::read_response_json(&mut app, req);

            let new_note_result = result.get(0).unwrap();
            assert_eq!(note_created.title, new_note_result.title);
            assert_eq!(note_created.content, new_note_result.content);
        })
    }

    #[test]
    fn test_delete_note_by_title_endpoint() {
        run_test(|| {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let pool = init_pool(&database_url).expect("Failed to create pool");
            let note_name: String = iter::repeat(())
                .map(|()| thread_rng().sample(Alphanumeric))
                .take(7)
                .collect();
            let note_created = create_note_post_request(&note_name, "bla");

            let mut app = test::init_service(
                App::new()
                    .data(pool.clone())
                    .wrap(Logger::default())
                    .service(web::resource("/api/notes/delete/{title}").to(delete_by_title))
                    .service(web::resource("/api/notes/{title}").to(get_notes_by_title)),
            );
            let uri = format!("/api/notes/delete/{}", note_created.title);
            let req = test::TestRequest::with_uri(uri.as_str()).to_request();
            let resp = test::block_on(app.call(req)).unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let uri = format!("/api/notes/{}", note_created.title);
            let req = test::TestRequest::with_uri(uri.as_str()).to_request();
            let result: Vec<NewNote> = test::read_response_json(&mut app, req);
            assert_eq!(0, result.len());
        })
    }
}
