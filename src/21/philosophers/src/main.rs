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


fn create_philosopher_thread(mutex: &mut Arc<Mutex<Vec<&'static str>>>, name: &'static str, left_fork: usize, right_fork: usize) -> thread::JoinHandle<()> {
    let forks = Arc::clone(&mutex);
    let needed_left_fork = format!("fork{}", left_fork);
    let needed_right_fork = format!("fork{}", right_fork);

    let t = thread::spawn(move || {
        println!("P{} needs left fork '{}' and right fork '{}'.", name, left_fork, right_fork);
        loop {
            let mut locked_forks = forks.lock().unwrap();
            let philosopher_left_fork = locked_forks[left_fork];
            let philosopher_right_fork = locked_forks[right_fork];

            if philosopher_left_fork == needed_left_fork{
                println!("P{} got left_fork: {}", name, philosopher_left_fork);
            } else {
                println!("P{} didn't get left_fork: {}", name, "other thing");
            }

            if philosopher_right_fork == needed_right_fork{
                println!("P{} got right_fork: {}", name, philosopher_right_fork);
            } else {
                println!("P{} didn't get right_fork: {}", name, "other thing");
            }

            println!("P{} current status: right_fork '{}', right_fork '{}'", name, philosopher_left_fork, philosopher_right_fork);
            break;
        }
    
    });
    return t;
}