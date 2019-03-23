extern crate rand;

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::{thread, time};

fn random_sleep(thread_name: String) {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1000, 10000);
    println!("{} will sleep: {}", thread_name, random);
    let sleep = time::Duration::from_millis(random);
    thread::sleep(sleep);
}

fn main() {
    println!("Hello, Producers and Consumers!");
    let number_of_producer_threads = 8;
    let number_of_consumer_threads = 8;

    let mutex: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let mut producer_thread_pool = vec![];
    let mut consumer_thread_pool = vec![];

    for i in 0..number_of_producer_threads {
        let queue = Arc::clone(&mutex);
        let t = thread::spawn(move || {
            random_sleep(format!("P{}", i));
            let string = String::from(format!("string P{}", i));
            println!("P{} producing string: {}", i, string);
            let mut locked_queue = queue.lock().unwrap();
            locked_queue.push(string);
            println!("P{} queue updated: {:?}", i, locked_queue);
        });
        producer_thread_pool.push(t);
    }

    for i in 0..number_of_consumer_threads {
        let queue = Arc::clone(&mutex);
        let t = thread::spawn(move || {
            random_sleep(format!("C{}", i));
            let mut locked_queue = queue.lock().unwrap();
            let consumed_string = locked_queue.pop();
            match consumed_string {
                Some(x) => {
                    println!("C{} consuming string: {:?}", i, x);
                }
                None => {
                    println!("C{} Got a None value!", i);
                }
            }
            println!("C{} queue updated: {:?}", i, locked_queue);
        });
        consumer_thread_pool.push(t);
    }

    for producer_handle in producer_thread_pool {
        producer_handle.join().unwrap();
    }

    for consumer_handle in consumer_thread_pool {
        consumer_handle.join().unwrap();
    }

    let queue = Arc::clone(&mutex);
    let locked_queue = queue.lock().unwrap();
    println!("Final queue state: {:?}", locked_queue);

}
