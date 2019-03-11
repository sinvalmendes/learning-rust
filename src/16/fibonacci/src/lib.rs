#![feature(test)]

extern crate test;

pub fn fibonacci(n: i32) -> i32{
    if n == 1 || n == 0 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::collections::HashMap;


    pub struct Fib {
        memo: HashMap<i32,i32>
    }

    impl Fib {

        pub fn new() -> Fib {
            Fib { memo: HashMap::new() }
        }


        pub fn fibonacci_memo(&mut self, n: i32) -> i32 {
            if n == 1 || n == 0 {
                self.memo.insert(n, 1);
            }
            let value = self.memo.get(&n);
            match value {
                Some(x) => {
                    return *x;
                },
                None    => {    
                    let result = self.fibonacci_memo(n - 1) + self.fibonacci_memo(n - 2);
                    self.memo.insert(n, result);
                    return result;
                }
            }
        }
    }

    #[bench]
    fn bench_normal_fib(b: &mut Bencher) { // using the test::Bencher to test the performance of the function
        fn bb() { test::black_box(fibonacci(20)); }
        b.iter(bb);
    }

    #[bench]
    fn bench_memoized_fib(b: &mut Bencher) { // using the test::Bencher to test the performance of the function
        let mut fib = Fib::new();

        b.iter(|| {
            for i in 1..1000 {
                fib.fibonacci_memo(20);
            }
        });
    }
}