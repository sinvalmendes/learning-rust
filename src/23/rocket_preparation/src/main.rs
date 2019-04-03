use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::{thread, time};

static FUEL: &str = "FUEL";
static OXI: &str = "OXIDIZER";
static ASTRO: &str = "ASTRONAUTS";

fn main() {
    println!("Hello, Rocket Preparation problem!");
    let resources = vec![FUEL, OXI, FUEL, ASTRO, ASTRO, OXI, OXI, FUEL]; // concrete vector to simplify implementation for now
    println!("Resources: {:?}", resources);

    let mut mutex: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(resources));
    let mut thread_pool = vec![];

    let r0 = create_rocket_thread(&mut mutex, "STS-1");
    let r1 = create_rocket_thread(&mut mutex, "Apollo 11");
    let r2 = create_rocket_thread(&mut mutex, "STS-66");

    thread_pool.push(r0);
    thread_pool.push(r1);
    thread_pool.push(r2);

    for thread in thread_pool {
        thread.join();
    }
}

fn create_producer_thread() {}

fn create_rocket_thread(
    mutex: &mut Arc<Mutex<Vec<&'static str>>>,
    name: &'static str,
) -> thread::JoinHandle<()> {
    let t = thread::spawn(move || {
        println!("Rocket: {}", name);
    });
    return t;
}
