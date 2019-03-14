#[macro_use]
extern crate criterion;
extern crate rand;

use std::collections::HashMap;
use criterion::Criterion;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


pub struct VecTest {
    memo: Vec<String>
}

impl VecTest {
    pub fn new() -> VecTest {
        VecTest { memo: Vec::new() }
    }

    pub fn insert(&mut self, k: String) {
        self.memo.push(k);
    }
}

pub struct HashTest {
    memo: HashMap<String, i32>
}


impl HashTest {

    pub fn new() -> HashTest {
        HashTest { memo: HashMap::new() }
    }

    pub fn get(&mut self, k: String) -> i32 {
        let result = self.memo.get(&k);
         match result {
            Some(x) => {
                return *x;
            },
            None    => { 
                return 0; 
            }
         }
    }

    pub fn insert(&mut self, k: String, v: i32) {
        self.memo.insert(k, v);
    }
}

fn criterion_benchmark1(c: &mut Criterion) {
    let mut fib = HashTest::new();
    c.bench_function("get key", move |b|
        b.iter(|| fib.get("key".to_string())));
}

fn criterion_benchmark2(c: &mut Criterion) {
    let mut fib = HashTest::new();
    c.bench_function("insert key", move |b|
        b.iter(|| fib.insert("key".to_string(), 0)));
}

fn criterion_benchmark3(c: &mut Criterion) {
    let mut fib = HashTest::new();
    fib.insert("key".to_string(), 0);
    c.bench_function("get existent key", move |b|
        b.iter(|| fib.get("key".to_string())));
}

fn criterion_benchmark4(c: &mut Criterion) {
    let mut fib = HashTest::new();
    fib.insert("key".to_string(), 0);
    for i in 1..1000 {
        let s: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        fib.insert(s.to_string(), i);
    }
    c.bench_function("get existent key in huge map", move |b|
        b.iter(|| fib.get("key".to_string())));
}

fn criterion_benchmark5(c: &mut Criterion) {
    let mut fib = HashTest::new();
    for i in 1..1000 {
        let s: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        fib.insert(s.to_string(), i);
    }
    c.bench_function("insert existent in huge map", move |b|
        b.iter(|| fib.insert("key".to_string(), 0)));
}

fn criterion_benchmark6(c: &mut Criterion) {
    let mut fib = VecTest::new();
    c.bench_function("insert in vec!", move |b|
        b.iter(|| fib.insert("key".to_string())));
}

fn criterion_benchmark7(c: &mut Criterion) {
    let mut fib = Vec::new();
    c.bench_function("insert in local vec!", move |b|
        b.iter(|| fib.push("key".to_string())));
}

criterion_group!(benches, criterion_benchmark1, criterion_benchmark2, 
    criterion_benchmark3, criterion_benchmark4, criterion_benchmark5,
    criterion_benchmark6, criterion_benchmark7);
criterion_main!(benches);