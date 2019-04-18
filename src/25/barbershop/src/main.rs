// A barbershop consists of a waiting room with n chairs, and the
// barber room containing the barber chair. If there are no customers
// to be served, the barber goes to sleep. If a customer enters the
// barbershop and all chairs are occupied, then the customer leaves
// the shop. If the barber is busy, but chairs are available, then the
// customer sits in one of the free chairs. If the barber is asleep, the
// customer wakes up the barber. Write a program to coordinate the
// barber and the customers.

extern crate rand;

use rand::Rng;
use std::{thread, time};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;


fn main() {
    println!("Hello, barber!");

    let mut barber_ready: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let mut barber_done: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let mut customer_ready: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));

    let mut thread_pool = vec![];

    let b = create_barber_thread(&mut barber_ready, &mut barber_done, &mut customer_ready, "Barber");
    thread_pool.push(b);

    for name in vec!["C1", "C2", "C3"] {
        let t = create_customer_thread(&mut barber_ready, &mut barber_done, &mut customer_ready, name);
        thread_pool.push(t);
    }

    for thread in thread_pool {
        thread.join().unwrap();
    }
}

fn create_barber_thread(barber_ready: &mut Arc<(Mutex<bool>, Condvar)>, barber_done: &mut Arc<(Mutex<bool>, Condvar)>,
                          customer_ready: &mut Arc<(Mutex<bool>, Condvar)>, name: &'static str) -> thread::JoinHandle<()> {
    let c_barber_ready = Arc::clone(barber_ready);
    let c_barber_done = Arc::clone(barber_done);
    let c_customer_ready = Arc::clone(customer_ready);

    let b = thread::spawn(move || {
        for _ in 0..3 {
            {
                let &(ref mtx, ref cnd) = &*c_customer_ready;
                let mut guard_customer_ready = mtx.lock().unwrap();
                println!("{}: waiting for customer {:?}", name, guard_customer_ready);
                guard_customer_ready = cnd.wait(guard_customer_ready).unwrap();
                println!("{}: customer appeared {:?}", name, guard_customer_ready);
            }
            {
                let &(ref mtx, ref cnd) = &*c_barber_ready;
                let guard_barber_ready = mtx.lock().unwrap();
                println!("{}: is ready {:?}", name, guard_barber_ready);
                cnd.notify_one();
            }
            sleep("Barber: working", 2000, 2001);
            {
                let &(ref mtx_barber_done, ref cnd) = &*c_barber_done;
                let guard_barber_done = mtx_barber_done.lock().unwrap();
                println!("{}: is finished haircut {:?}", name, guard_barber_done);
                cnd.notify_one();
            }
        }
    });
    return b;
}

fn create_customer_thread(barber_ready: &mut Arc<(Mutex<bool>, Condvar)>, barber_done: &mut Arc<(Mutex<bool>, Condvar)>,
                          customer_ready: &mut Arc<(Mutex<bool>, Condvar)>, name: &'static str) -> thread::JoinHandle<()> {
    let c_barber_ready = Arc::clone(barber_ready);
    let c_barber_done = Arc::clone(barber_done);
    let c_customer_ready = Arc::clone(customer_ready);

    let t = thread::spawn(move || {
        loop {
            {
                let &(ref mtx, ref cnd) = &*c_customer_ready;
                let guard_customer_ready = mtx.lock().unwrap();
                println!("{}: notifying barber that I'm ready {:?}", name, guard_customer_ready);
                cnd.notify_one();
            }
            {
                let &(ref mtx, ref cnd) = &*c_barber_ready;
                let guard_barber_ready = mtx.lock().unwrap();
                println!("{}: waiting for barber {:?}", name, guard_barber_ready);
                let guard_barber_ready = cnd.wait_timeout(
                    guard_barber_ready, Duration::from_millis(1000)).unwrap();
                if guard_barber_ready.1.timed_out() {
                    println!("{}: timeout {:?}", name, guard_barber_ready);
                    continue;
                }
                println!("{}: after wait {:?}", name, guard_barber_ready);
                break;
            }
        }
        println!("{}: ready to get haircut", name);
        {
            let &(ref mtx, ref cnd) = &*c_barber_done;
            let mut guard_barber_done = mtx.lock().unwrap();
            println!("{}: waiting for barber to finish my haircut {:?}", name, guard_barber_done);
            guard_barber_done = cnd.wait(guard_barber_done).unwrap();
            println!("{}: finished my haircut, I can leave now {:?}", name, guard_barber_done);
        }
    });
    return t;
}

fn sleep(thread_name: &'static str, min: u64, max: u64) {
    let random = rand::thread_rng().gen_range(min, max);
    println!("{}: sleeping for: {} milliseconds", thread_name, random);
    thread::sleep(time::Duration::from_millis(random));
}
