extern crate rand;

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::{thread, time};

fn main() {
    println!("Hello, philosophers!");

    let number_of_times = 5;
    let forks = vec!["fork", "fork", "fork", "fork", "fork"];

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
    let mut forks = Arc::clone(&mutex);

    let t = thread::spawn(move || {
        println!("{} needs left fork '{}' and right fork '{}'.", name, left_fork, right_fork);
        let mut got_left_fork = false;
        loop {
            if !(got_left_fork) {
                think(name);
                println!("{} will attempt to get left fork!", name);
                got_left_fork = try_get_fork(&mut forks, left_fork);
            }

            if (got_left_fork) {
                println!("{} got left fork!", name);
                think(name);
                println!("{} will attempt to get right fork!", name);
                let got_right_fork = try_get_fork(&mut forks, right_fork);
                if (got_right_fork) {
                    println!("{} got right fork!", name);
                    println!("{} CAN EAT!", name);
                    println!("{} EATING!", name);
                    think(name);
                    let mut locked_forks = forks.lock().unwrap();
                    locked_forks[right_fork] = "fork";
                    locked_forks[left_fork] = "fork";
                    break;
                    // got_left_fork = false;
                    // needes to give back the right fork
                    // needes to give back the left fork
                }
            }
        }
    });
    return t;
}

fn try_get_fork(forks: &mut Arc<Mutex<Vec<&'static str>>>, fork_index:usize) -> bool {
    let mut locked_forks = forks.lock().unwrap();
    let philosopher_fork = locked_forks[fork_index];
    if philosopher_fork == "fork" {
        locked_forks[fork_index] = "None";
        return true;
    } else {
        return false;
    }
}

fn think(thread_name: &'static str) {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1000, 10000);
    println!("{} thinking for: {} milliseconds", thread_name, random);
    let sleep = time::Duration::from_millis(random);
    thread::sleep(sleep);
}
