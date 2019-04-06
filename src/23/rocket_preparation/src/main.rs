extern crate rand;

use rand::Rng;
use std::sync::{Arc, Barrier, Mutex};
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
    let lift_off_arc = Arc::new(Barrier::new(4));

    let mut mutex: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(resources));
    let mut thread_pool = vec![];

    let lift_off0 = Arc::clone(&lift_off_arc);
    let r0 = create_rocket_thread(&mut mutex, "STS-1", lift_off0);
    thread_pool.push(r0);

    let lift_off1 = Arc::clone(&lift_off_arc);
    let r1 = create_rocket_thread(&mut mutex, "Apollo 11", lift_off1);
    thread_pool.push(r1);

    let lift_off2 = Arc::clone(&lift_off_arc);
    let r2 = create_rocket_thread(&mut mutex, "STS-66", lift_off2);
    thread_pool.push(r2);

    sleep("Main", 10000, 10001);
    lift_off_arc.wait();

    for thread in thread_pool {
        thread.join().unwrap();
    }

}

fn control_room_thread() {
    
}

fn create_rocket_thread(
    mutex: &mut Arc<Mutex<Vec<&'static str>>>, name: &'static str, lift_off: Arc<Barrier>) -> thread::JoinHandle<()> {
    let mut produced_resources = Arc::clone(&mutex);

    let t = thread::spawn(move || {
        println!("Rocket: {}", name);
        let necessary_rocket_resources = vec![FUEL, OXI, ASTRO];

        let mut necessary_rocket_index = 0;
        while necessary_rocket_index < 3 {
            let mut i = 0;
            let number_of_resources = produced_resources.clone().lock().unwrap().len();
            while i < number_of_resources {
                let mut locked_resources = produced_resources.lock().unwrap();
                if locked_resources[i] == necessary_rocket_resources[necessary_rocket_index] {
                    necessary_rocket_index += 1;
                    let consumed_resource = locked_resources[i];
                    println!("Rocket: {} consumed {}", name, consumed_resource);
                    locked_resources[i] = NONE;
                    break;
                }
                i += 1;
            }
            sleep(name, 1000, 2000);
        }
        lift_off.wait();
        println!("Rocket: {} LIFT OFF", name);
    });
    return t;
}

fn sleep(thread_name: &'static str, min: u64, max: u64) {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(min, max);
    println!("{} sleeping for: {} milliseconds", thread_name, random);
    let sleep = time::Duration::from_millis(random);
    thread::sleep(sleep);
}
