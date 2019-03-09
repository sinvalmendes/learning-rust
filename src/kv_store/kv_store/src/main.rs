#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[cfg(test)] mod tests;

mod db;
use db::memory::MemoryDB;
use std::sync::RwLock;
use rocket::State;
use rocket::response::content;
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
fn get_value(db: State<RwLock<MemoryDB>>, key: String) -> content::Json<String> {
    let db = db.read().unwrap();
    let result = db.get_value(&key);

    match result {
        Ok(x) => {
            let json_result = json!({
                key: x,
            });
            return content::Json(format!("{}", json_result));
        },
        Err(e) => {
            let json_result = json!({
                "error": format!("{}", e),
            });
            return content::Json(format!("{}", json_result));
        }
    }
}

#[put("/api/kv/<key>", format = "json", data = "<kv>")]
fn put_kv(db: State<RwLock<MemoryDB>>, key: String, kv: Json<KV>) -> content::Json<String> {
    let mut db = db.write().unwrap();
    let value = &kv.value;
    let result = db.store_kv(key, value.to_string());
    match result {
        Ok(x)  => {
            let json_result = json!({
                "result": true,
            });
            return content::Json(format!("{}", json_result));
        },
        Err(e) => {
            let json_result = json!({
                "error": format!("{}", e),
            });
            return content::Json(format!("{}", json_result));
        }
    }
}

fn get_rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(RwLock::new(MemoryDB::new()))
        .mount("/", routes![index, get_value, put_kv])
}

fn main() {
    get_rocket().launch();
}
