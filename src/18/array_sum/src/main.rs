#[cfg(test)]
mod tests;

use std::sync::{Arc, Mutex};
use std::thread;
use std::vec::Vec;

pub struct Sum {
    array: Vec<i32>,
}

impl Sum {
    fn new() -> Sum {
        Sum { array: Vec::new() }
    }

    fn populate_array(&mut self) {
        for i in 1..10000 {
            self.array.push(i);
        }
    }

    fn sum_iter(self) -> i32 {
        let mut result = 0;
        for i in self.array.into_iter() {
            result += i;
        }
        return result;
    }

    fn sum_concurrent(self) -> i32 {
        let mut result = 0;
        let mut handles = vec![];
        let mutex_array = Arc::new(Mutex::new(self.array));
        let mutex = Arc::new(Mutex::new(0));

        let c_mutex = Arc::clone(&mutex);
        let a_mutex = Arc::clone(&mutex_array);

        let t1 = thread::spawn(move || {
            let mut count = c_mutex.lock().unwrap();
            let mut array = a_mutex.lock().unwrap();
            for i in 0..5000 {
                *count += array[i];
                println!("t1 {:?}", array[i]);
            }
        });
        handles.push(t1);

        let c_mutex = Arc::clone(&mutex);
        let a_mutex = Arc::clone(&mutex_array);

        let t2 = thread::spawn(move || {
            let mut count = c_mutex.lock().unwrap();
            let mut array = a_mutex.lock().unwrap();
            for i in 5000..9999 {
                *count += array[i];
                println!("t2 {:?}", array[i]);
            }
        });
        handles.push(t2);

        for handle in handles {
            handle.join().unwrap();
        }
        let count = mutex.lock().unwrap();

        println!("t1_result---> {}", count);
        return count.to_string().parse::<i32>().unwrap();
    }
}

pub fn main() {
    println!("Sum!");
}
