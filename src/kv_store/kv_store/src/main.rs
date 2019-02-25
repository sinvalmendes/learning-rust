#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket; // https://doc.rust-lang.org/1.7.0/book/macros.html
#[macro_use] extern crate serde_derive;

mod db;
use db::memory::MemoryDB;
use std::sync::RwLock;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::json;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct KV {
    value: String,
}

#[get("/api")]
fn index(db: State<RwLock<MemoryDB>>) -> String {
    let db = db.read().unwrap();
    return format!("{:?}", db.key_values);
}

#[get("/api/kv/<key>")]
fn get_value(db: State<RwLock<MemoryDB>>, key: String) -> String {
    let db = db.read().unwrap();
    let result = db.get_value(&key);

    match result {
        Ok(x) => {
            let json_result = json!({
                key: x,
            });
            return format!("{}", json_result);
        },
        Err(e) => {
            let json_result = json!({
                "result": e,
            });
            return format!("{}", json_result);
        }
    }
}

#[put("/api/kv/<key>", format = "json", data = "<kv>")]
fn post_kv(db: State<RwLock<MemoryDB>>, key: String, kv: Json<KV>) -> String { 
    let mut db = db.write().unwrap();
    let value = &kv.value;
    let result = db.store_kv(key, value.to_string());
    match result {
        Ok(x)  => {
            let json_result = json!({
                "result": true,
            });
            return format!("{}", json_result);
        },
        Err(e) => {
            let json_result = json!({
                "error": format!("{}", e),
            });
            return format!("{}", json_result);
        }
    }
}

fn main() {
    rocket::ignite()
        .manage(RwLock::new(MemoryDB::new()))
        .mount("/", routes![index, get_value, post_kv])
        .launch();
}
