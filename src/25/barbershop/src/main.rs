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

    let barber_ready: Arc<(Mutex<(bool, u16)>, Condvar)> = Arc::new((Mutex::new((false, 0)), Condvar::new()));
    let barber_done: Arc<(Mutex<(bool, u16)>, Condvar)> = Arc::new((Mutex::new((false, 0)), Condvar::new()));
    let customer_ready: Arc<(Mutex<(bool, u16)>, Condvar)> = Arc::new((Mutex::new((false, 0)), Condvar::new()));

    let mut thread_pool = vec![];
    for name in vec!["C1", "C2", "C3"] {
        let mut clone_barber_ready = Arc::clone(&barber_ready);
        let mut clone_barber_done = Arc::clone(&barber_done);
        let mut clone_customer_ready = Arc::clone(&customer_ready);

        let t = create_customer_thread(&mut clone_barber_ready, &mut clone_barber_done, &mut clone_customer_ready, name);
        thread_pool.push(t);
    }

    let b = thread::spawn(move || {
        for i in 0..3 {
            {
                let &(ref mtx, ref cnd) = &*customer_ready;
                let mut guard = mtx.lock().unwrap();
                println!("Baber: waiting for customer {:?}", guard);
                guard = cnd.wait(guard).unwrap();
                println!("Baber: customer appeared {:?}", guard);
            }
            {
                let &(ref mtx, ref cnd) = &*barber_ready;
                let mut guard = mtx.lock().unwrap();
                println!("Barber: woke up {:?}", guard);
                guard.1 = guard.1.wrapping_add(1);
                guard.0 = true;
                cnd.notify_one();
            }
            sleep("Barber: working", 2000, 2001);
            {
                let &(ref mtx_barber_done, ref cnd) = &*barber_done;
                let mut guard_done = mtx_barber_done.lock().unwrap();
                println!("Barber: is done {:?}", guard_done);
                guard_done.1 = guard_done.1.wrapping_add(1);
                guard_done.0 = true;
                cnd.notify_one();
            }
        }
    });
    thread_pool.push(b);

    for thread in thread_pool {
        thread.join().unwrap();
    }
}

fn create_customer_thread(barber_ready: &mut Arc<(Mutex<(bool, u16)>, Condvar)>, barber_done: &mut Arc<(Mutex<(bool, u16)>, Condvar)>,
                          customer_ready: &mut Arc<(Mutex<(bool, u16)>, Condvar)>, name: &'static str) -> thread::JoinHandle<()> {
    let c_barber_ready = Arc::clone(barber_ready);
    let c_barber_done = Arc::clone(barber_done);
    let c_customer_ready = Arc::clone(customer_ready);

    let t = thread::spawn(move || {
        loop {
            {
                let &(ref mtx, ref cnd) = &*c_customer_ready;
                let mut guard = mtx.lock().unwrap();
                println!("Client {}: notifying barber that I'm ready {:?}", name, guard);
                guard.1 = guard.1.wrapping_add(1);
                guard.0 = true;
                cnd.notify_one();
            }
            {
                let &(ref mtx, ref cnd) = &*c_barber_ready;
                let mut guard = mtx.lock().unwrap();
                println!("Client {}: waiting for barber {:?}", name, guard);
                let guard = cnd.wait_timeout(
                    guard, Duration::from_millis(1000)).unwrap();
                if guard.1.timed_out() {
                    println!("Client {}: timeout {:?}", name, guard);
                    continue;
                }
                println!("Client {}: after wait {:?}", name, guard);
                break;
            }
        }
        println!("Client {} ready to hair cut", name);
        {
            let &(ref mtx, ref cnd) = &*c_barber_done;
            let mut guard = mtx.lock().unwrap();
            println!("Client {}: waiting for barber to finish the hair cut {:?}", name, guard);
            guard = cnd.wait(guard).unwrap();
            println!("Client {}: finished the hair cut, I can leave now {:?}", name, guard);
        }    
    });
    return t;
}

fn sleep(thread_name: &'static str, min: u64, max: u64) {
    let random = rand::thread_rng().gen_range(min, max);
    println!("{}: sleeping for: {} milliseconds", thread_name, random);
    thread::sleep(time::Duration::from_millis(random));
}
