extern crate rand;

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::{thread, time};

fn main() {
    println!("Hello, philosophers!");

    let number_of_times = 5;
    let forks = vec!["fork0", "fork1", "fork2", "fork3", "fork4"];

    let mut mutex: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(forks));
    let mut philosophers_thread_pool = vec![];

    let p0 = create_philosopher_thread(&mut mutex, "0", 0, 1);
    let p1 = create_philosopher_thread(&mut mutex, "1", 1, 2);
    let p2 = create_philosopher_thread(&mut mutex, "2", 2, 3);
    let p3 = create_philosopher_thread(&mut mutex, "3", 3, 4);
    let p4 = create_philosopher_thread(&mut mutex, "4", 4, 0);

    philosophers_thread_pool.push(p0);
    philosophers_thread_pool.push(p1);
    philosophers_thread_pool.push(p2);
    philosophers_thread_pool.push(p3);
    philosophers_thread_pool.push(p4);

    for philosopher_handle in philosophers_thread_pool {
        philosopher_handle.join().unwrap();
    }
}


fn create_philosopher_thread(mutex: &mut Arc<Mutex<Vec<&'static str>>>, name: &'static str, left_fork: u8, right_fork: u8) -> thread::JoinHandle<()> {
    let forks_clone = Arc::clone(&mutex);
    let t = thread::spawn(move || {
        println!("P{} needs left fork '{}' and right fork '{}'.", name, left_fork, right_fork);
    });
    return t;
}