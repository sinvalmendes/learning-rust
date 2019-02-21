use std::sync::{Arc, Mutex};
use std::thread;


fn main() {

    let mutex = Arc::new(Mutex::new(0)); // this is the Mutex inside Arc (Atomic reference)
    
    // here we are going to declare two independent threads, t1 and t2
    let c_mutex = Arc::clone(&mutex); // here we are cloning the Mutex so we can move to the next thread
    let t1 = thread::spawn(move || { // t1
        let mut count = c_mutex.lock().unwrap();
        println!("-----------------------------");
        println!("t1 got the lock, will increment count (current value: {})", count);
        *count += 1;
        println!("t1 updated count: {}", count);
        println!("-----------------------------");
    });

    let c_mutex = Arc::clone(&mutex);
    let t2 = thread::spawn(move || { // t2
        let mut count = c_mutex.lock().unwrap();
        println!("-----------------------------");
        println!("t2 got the lock, will increment count (current value: {})", count);
        *count += 1;
        println!("t2 updated count: {}", count);
        println!("-----------------------------");
    });

    // now we are creating a vector to store all the threads to make sure all of them execute before the program exit
    let mut handles = vec![];
    handles.push(t1);
    handles.push(t2);

    // here we are leveraging the for statement to automate the creation of 8 threads, just like t1 and t2
    for i in 2..10 {
        let c_mutex = Arc::clone(&mutex); // here we are cloning the Mutex so we can move to the next thread
        let t = thread::spawn(move || { // t
            let mut count = c_mutex.lock().unwrap();
            println!("-----------------------------");
            println!("t{} got the lock, will increment count (current value: {})", i, count);
            *count += 1;
            println!("t{} updated count: {}", i, count);
            println!("-----------------------------");
        });
        handles.push(t);
    }

    // making sure all the threads executes before the end of this main() scope
    for handle in handles {
        handle.join().unwrap();
    }

    let count = mutex.lock().unwrap();
    println!("Hello, concurrency! {}", count); // you should see 10 here
}
