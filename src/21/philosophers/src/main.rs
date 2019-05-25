extern crate rand;

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::{thread, time};

static FORK: &str = "fork";

fn main() {
    println!("Hello, philosophers!");

    let forks = vec![FORK, FORK, FORK, FORK, FORK];

    let mut mutex: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(forks));
    let mut philosophers_thread_pool = vec![];

    philosophers_thread_pool.push(create_philosopher_thread(&mut mutex, "P0", 0, 1));
    philosophers_thread_pool.push(create_philosopher_thread(&mut mutex, "P1", 1, 2));
    philosophers_thread_pool.push(create_philosopher_thread(&mut mutex, "P2", 2, 3));
    philosophers_thread_pool.push(create_philosopher_thread(&mut mutex, "P3", 3, 4));
    philosophers_thread_pool.push(create_philosopher_thread(&mut mutex, "P4", 4, 0));

    for philosopher_handle in philosophers_thread_pool {
        philosopher_handle.join().unwrap();
    }
}

fn create_philosopher_thread(mutex: &mut Arc<Mutex<Vec<&'static str>>>, name: &'static str, left_fork_index: usize, right_fork_index: usize) -> thread::JoinHandle<()> {
    let mut forks = Arc::clone(&mutex);
    let t = thread::spawn(move || {
        println!("{} needs left fork at '{}' and right fork at '{}'.", name, left_fork_index, right_fork_index);
        let mut got_left_fork = false;
        loop {
            if !got_left_fork {
                got_left_fork = try_get_fork(&mut forks, left_fork_index, name, "left");
            }

            if got_left_fork {
                println!("{} got left fork!", name);
                let got_right_fork = try_get_fork(&mut forks, right_fork_index, name, "right");
                if got_right_fork {
                    println!("{} got right fork!", name);
                    println!("{} EATING!", name);
                    think(name);
                    let mut locked_forks = forks.lock().unwrap();
                    locked_forks[left_fork_index] = FORK;
                    locked_forks[right_fork_index] = FORK;
                    break;
                }
                let mut locked_forks = forks.lock().unwrap();
                locked_forks[left_fork_index] = FORK;
            }
        }
    });
    return t;
}

fn try_get_fork(forks: &mut Arc<Mutex<Vec<&'static str>>>, fork_index:usize, thread_name: &str, fork_type: &str) -> bool {
    think(thread_name);
    println!("{} will attempt to get {} fork!", thread_name, fork_type);
    let mut locked_forks = forks.lock().unwrap();
    if locked_forks[fork_index] == FORK {
        locked_forks[fork_index] = "None";
        return true;
    }
    return false;
}

fn think(thread_name: & str) {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1000, 5000);
    println!("{} thinking for: {} milliseconds", thread_name, random);
    let sleep = time::Duration::from_millis(random);
    thread::sleep(sleep);
}
