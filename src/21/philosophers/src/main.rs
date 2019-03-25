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
    let needed_left_fork = "fork";
    let needed_right_fork = "fork";
    let mut forks = Arc::clone(&mutex);

    let t = thread::spawn(move || {
        println!("{} needs left fork '{}' and right fork '{}'.", name, left_fork, right_fork);
        let mut got_left_fork = false;
        loop {

            if !(got_left_fork) {
                think(name);
                got_left_fork = try_get_left_fork(name, &mut forks, left_fork, needed_left_fork);
            }

            if (got_left_fork) {
                think(name);
                let got_right_fork = try_get_right_fork(name, &mut forks, right_fork, needed_right_fork);
                if (got_right_fork) {
                    println!("{} CAN EAT!", name);
                    println!("{} EATING!", name);
                    think(name);
                    let mut locked_forks = forks.lock().unwrap();
                    locked_forks[right_fork] = needed_right_fork;
                    locked_forks[left_fork] = needed_left_fork;
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

// refactor both methods into just one
fn try_get_right_fork(name: &'static str, forks: &mut Arc<Mutex<Vec<&'static str>>>, right_fork:usize, needed_right_fork: &'static str) -> bool {
    let mut locked_forks = forks.lock().unwrap();
    let philosopher_right_fork = locked_forks[right_fork]; // extract method and lock only when needed
    let got_right_fork = philosopher_right_fork == needed_right_fork; // extract method
    if got_right_fork { // extract method
        println!("{} got right_fork: {}", name, philosopher_right_fork);
        locked_forks[right_fork] = "None";
        return true;
    } else {
        println!("{} didn't get right_fork: {}", name, philosopher_right_fork);
        return false;
    }
}

fn try_get_left_fork(name: &'static str, forks: &mut Arc<Mutex<Vec<&'static str>>>, left_fork:usize, needed_left_fork: &'static str) -> bool {
    let mut locked_forks = forks.lock().unwrap();
    let philosopher_left_fork = locked_forks[left_fork]; // extract method and lock only when needed
    let got_left_fork = philosopher_left_fork == needed_left_fork; // extract method
    if got_left_fork { // extract method
        println!("{} got left_fork: {}", name, philosopher_left_fork);
        locked_forks[left_fork] = "None";
        return true;
    } else {
        println!("{} didn't get left_fork: {}", name, philosopher_left_fork);
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
