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

    let p0 = create_philosopher_thread(&mut mutex, "P0", 0, 1);
    let p1 = create_philosopher_thread(&mut mutex, "P1", 1, 2);
    let p2 = create_philosopher_thread(&mut mutex, "P2", 2, 3);
    let p3 = create_philosopher_thread(&mut mutex, "P3", 3, 4);
    let p4 = create_philosopher_thread(&mut mutex, "P4", 4, 0);

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
        println!("{} needs left fork '{}' and right fork '{}'.", name, left_fork, right_fork);
        loop {
            let mut locked_forks = forks.lock().unwrap();
            let philosopher_left_fork = locked_forks[left_fork];
            let philosopher_right_fork = locked_forks[right_fork];

            let got_left_fork = philosopher_left_fork == needed_left_fork;
            // needs to add thinking step here
            let got_right_fork = philosopher_right_fork == needed_right_fork;

            if got_left_fork {
                println!("{} got left_fork: {}", name, philosopher_left_fork);
                locked_forks[left_fork] = "None";
            } else {
                println!("{} didn't get left_fork: {}", name, philosopher_left_fork);
            }

            if got_right_fork {
                println!("{} got right_fork: {}", name, philosopher_right_fork);
                locked_forks[right_fork] = "None";
            } else {
                println!("{} didn't get right_fork: {}", name, philosopher_right_fork);
            }

            if (got_left_fork == got_right_fork) == true {
                println!("{} CAN EAT!: right_fork '{}', right_fork '{}'", name, philosopher_left_fork, philosopher_right_fork);
                println!("{} EATING!", name);
            }
            locked_forks[left_fork] = philosopher_left_fork;
            // needs to add thinking step here
            locked_forks[right_fork] = philosopher_right_fork;
            std::mem::drop(locked_forks);

            break;
        }
    
    });
    return t;
}


fn think(thread_name: String) {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1000, 10000);
    println!("{} thinking for: {} milliseconds", thread_name, random);
    let sleep = time::Duration::from_millis(random);
    thread::sleep(sleep);
}
