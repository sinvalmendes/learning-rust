extern crate rand;

use std::{thread, time};

use rand::Rng;

use std::sync::{Arc, Condvar, Mutex};


fn main() {
    println!("Hello, world!");

    let mutcond: Arc<(Mutex<(bool, u16)>, Condvar)> =
        Arc::new((Mutex::new((false, 0)), Condvar::new()));

    let mut threads = vec![];
    let names = vec!["0", "1", "2"];
    for name in names {
        let mutcond = Arc::clone(&mutcond);
        let t = thread::spawn(move || {
            let &(ref mtx, ref cnd) = &*mutcond;
            let mut guard = mtx.lock().unwrap();
            while !guard.0 {
                println!("thread {} guard {:?}", name, guard);
                guard = cnd.wait(guard).unwrap();
                println!("thread {} guard {:?}", name, guard);
            }
            println!("Started execution of thread {}", name);
        });
        threads.push(t);
    }

    println!("Main will release!");
    sleep("Main", 2000, 2001);
    {
        let &(ref mtx, ref cnd) = &*mutcond;
        let mut guard = mtx.lock().unwrap();
        guard.1 = guard.1.wrapping_add(1);
        guard.0 = true;
        cnd.notify_all();
    }
    println!("Main released!");

    for thread in threads {
        thread.join().unwrap();
    }
}

fn sleep(thread_name: &'static str, min: u64, max: u64) {
    let random = rand::thread_rng().gen_range(min, max);
    println!("{} sleeping for: {} milliseconds", thread_name, random);
    thread::sleep(time::Duration::from_millis(random));
}
