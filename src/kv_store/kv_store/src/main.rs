#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
mod db;
use db::memory::MemoryDB;

use std::sync::RwLock;
use rocket::State;

#[get("/api")]
fn index(db: State<RwLock<MemoryDB>>) -> String {
    let db = db.read().unwrap();
    return format!("{:?}", db.key_values);
}

#[get("/api/kv/<name>")]
fn get_value(db: State<RwLock<MemoryDB>>, name: String) -> String {
    let db = db.read().unwrap();
    let result = db.get_value(name);
    match result {
        Ok(x)  => {
            println!("{}", x);
            return format!("Value: {}", x); // return JSON data
        },
        Err(e) => return format!("{}", e) // return JSON data
    }
}

#[get("/api/kv/<key>/<value>")] // use POST and JSON data
fn post_kv(db: State<RwLock<MemoryDB>>, key: String, value: String) -> String {
    let mut db = db.write().unwrap();
    let result = db.store_kv(key, value);
    match result {
        Ok(x)  => {
            println!("{}", x);
            return format!("The KV was stored!"); // return JSON data
        },
        Err(e) => return format!("{}", e) // return JSON data
    }
}

fn main() {
    rocket::ignite()
        .manage(RwLock::new(MemoryDB::new()))
        .mount("/", routes![index, get_value, post_kv])
        .launch();
}
