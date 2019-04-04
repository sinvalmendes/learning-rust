extern crate rand;

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::{thread, time};

static FUEL: &str = "FUEL";
static OXI: &str = "OXIDIZER";
static ASTRO: &str = "ASTRONAUTS";

static NONE: &str = "NONE";

fn main() {
    println!("Hello, Rocket Preparation problem!");
    let resources = vec![FUEL, OXI, FUEL, ASTRO, ASTRO, OXI, OXI, FUEL, ASTRO]; // concrete vector to simplify implementation for now
    println!("Resources: {:?}", resources);

    let mut mutex: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(resources));
    let mut thread_pool = vec![];

    let r0 = create_rocket_thread(&mut mutex, "STS-1");
    thread_pool.push(r0);

    let r1 = create_rocket_thread(&mut mutex, "Apollo 11");
    thread_pool.push(r1);

    let r2 = create_rocket_thread(&mut mutex, "STS-66");
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
    let mut produced_resources = Arc::clone(&mutex);

    let t = thread::spawn(move || {
        println!("Rocket: {}", name);
        let necessary_rocket_resources = vec![FUEL, OXI, ASTRO];

        let mut necessary_rocket_index = 0;
        while necessary_rocket_index < 3 {
            let mut locked_resources = produced_resources.lock().unwrap();
            let mut i = 0;
            while i < locked_resources.len() {
                if locked_resources[i] == necessary_rocket_resources[necessary_rocket_index] {
                    necessary_rocket_index += 1;
                    let consumed_resource = locked_resources[i];
                    println!("Rocket: {} consumed {}", name, consumed_resource);
                    locked_resources[i] = NONE;
                    break;
                }
                i += 1;
            }
            sleep(name);
        }
    });
    return t;
}

fn sleep(thread_name: &'static str) {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1000, 5000);
    println!("{} sleeping for: {} milliseconds", thread_name, random);
    let sleep = time::Duration::from_millis(random);
    thread::sleep(sleep);
}
