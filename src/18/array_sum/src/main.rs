#[cfg(test)]
mod tests;

use std::time::{Instant};


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
        for i in 0..10000 {
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

    fn sum_concurrent(self) -> std::vec::Vec<thread::JoinHandle<()>> {
        let _result = 0;
        let array_len = self.array.len();
        let mut handles = vec![];
        // let mutex_array = Arc::new(Mutex::new(self.array));
        let mutex = Arc::new(Mutex::new(0));

        let mut k = 0;
        for i in 1..9 {
            let slice = array_len/8;
            let offset = (slice * i) - 1;

            let thread_array = self.array.clone();
            let c_mutex = Arc::clone(&mutex);
            // let a_mutex = Arc::clone(&mutex_array);
            let t1 = thread::spawn(move || {
                // let array = a_mutex.lock().unwrap();
                let mut thread_internal_count = 0;
                for j in k..offset {
                    thread_internal_count += thread_array[j];
                }
                let mut count = c_mutex.lock().unwrap();
                *count += thread_internal_count;
            });
            k = offset + 1;
            handles.push(t1);
        }
        return handles;
    }
}


fn benchmark1() {
    let bench_name = "benchmark conc1";
    let mut sum = Sum::new();
    sum.populate_array();
    let handles = sum.sum_concurrent();
    let now = Instant::now();
    for handle in handles {
        handle.join().unwrap();
    }
    let elapsed = now.elapsed().as_nanos();
    println!("{}: {}", bench_name, elapsed);
}

fn benchmark2 () {
    let bench_name = "benchmark iter1";
    let mut sum = Sum::new();
    sum.populate_array();
    let now = Instant::now();
    sum.sum_iter();
    let elapsed = now.elapsed().as_nanos();
    println!("{}: {}", bench_name, elapsed);
}

pub fn main() {
    benchmark1();
    benchmark2();
}
