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

    fn sum_concurrent(self) -> i32 {
        return 0;
    }

    fn get_thread_pool(self, number_of_threads: i32, slice:i32) -> std::vec::Vec<thread::JoinHandle<()>> {
        let mut threads = vec![];
        let mutex = Arc::new(Mutex::new(0));

        let mut k = 0;
        for i in 0..number_of_threads {
            let offset = slice * (i + 1);

            let thread_array = self.array.clone();
            let counter_mutex = Arc::clone(&mutex);
            let t = thread::spawn(move || {
                let mut thread_internal_count = 0;
                for j in k..offset {
                    thread_internal_count += thread_array[j as usize];
                }
                let mut count = counter_mutex.lock().unwrap();
                *count += thread_internal_count;
            });
            k = offset + 1;
            threads.push(t);
        }
        return threads;
    }

    fn get_sum_thread_pool(self) -> std::vec::Vec<thread::JoinHandle<()>> {
        let mut number_of_threads: i32 = 10;
        let array_len: i32 = self.array.len() as i32;

        if self.array.len() % 10 != 0 {
            number_of_threads = 1;
        }
        let slice: i32 = array_len/number_of_threads;

        return self.get_thread_pool(number_of_threads, slice);
    }
}


fn benchmark1() {
    let bench_name = "benchmark conc1";
    let mut sum = Sum::new();
    sum.populate_array();
    let thread_pool = sum.get_sum_thread_pool();
    let now = Instant::now();
    for handle in thread_pool {
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
    let result = sum.sum_iter();
    let elapsed = now.elapsed().as_nanos();
    println!("{}:, result:{}, elapsed time:{}", bench_name, result, elapsed);
}

pub fn main() {
    benchmark1();
    benchmark2();
}
