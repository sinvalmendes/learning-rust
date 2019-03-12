#[macro_use]
extern crate criterion;

use std::collections::HashMap;
use criterion::Criterion;

pub struct Fib {
    memo: HashMap<i32,i32>
}

impl Fib {

    pub fn new() -> Fib {
        Fib { memo: HashMap::new() }
    }

    pub fn fibonacci(&mut self, n: i32) -> i32 {
        if n == 1 || n == 0 {
            return 1;
        }
        return self.fibonacci(n - 1) + self.fibonacci(n - 2);
    }

    pub fn fibonacci_memo(&mut self, n: i32) -> i32 {
        if n == 1  {
            return 1;
        } else if n == 0 {
            return 0;
        }
        let value = self.memo.get(&n);
        if value != None {
            return *value.unwrap();
        } else {
            let result = self.fibonacci_memo(n - 1) + self.fibonacci_memo(n - 2);
            self.memo.insert(n, result);
            return result;
        }
    }

    pub fn fibonacci_memo2(&mut self, n: i32) -> i32 {
        if n == 1  {
            return 1;
        } else if n == 0 {
            return 0;
        }

        let value = self.memo.get(&n);
        match value {
            Some(x) => {
                return *x;
            },
            None    => {
                let result = self.fibonacci_memo2(n - 1) + self.fibonacci_memo2(n - 2);
                self.memo.insert(n, result);
                return result;
            }
        }
    }

    pub fn fibonacci_iterative(&mut self, n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut c = 0;

        if n == 0 {
            return 0
        }

        for _ in 0..(n+1) {
            c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    let mut fib = Fib::new();
    // let result = fib.fibonacci_memo2(10);
    // println!("{}", result);
    c.bench_function("fibonacci_memo 35", move |b| b.iter(|| fib.fibonacci_memo(35)));
}

fn criterion_benchmark2(c: &mut Criterion) {
    let mut fib = Fib::new();
    c.bench_function("fibonacci_memo2 35", move |b| b.iter(|| fib.fibonacci_memo2(35)));
}

fn criterion_benchmark3(c: &mut Criterion) {
    let mut fib = Fib::new();
    c.bench_function("fibonacci 35", move |b| b.iter(|| fib.fibonacci(35)));
}

fn criterion_benchmark4(c: &mut Criterion) {
    let mut fib = Fib::new();
    c.bench_function("fibonacci_iter 35", move |b| b.iter(|| fib.fibonacci_iterative(35)));
}
criterion_group!(benches, criterion_benchmark, criterion_benchmark2, criterion_benchmark3, criterion_benchmark4);
criterion_main!(benches);