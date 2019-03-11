use std::collections::HashMap;

pub struct Fib {
    memo: HashMap<i32,i32>
}

fn main() {
    println!("Hello, world!");
    println!("fibonacci(5): {}", fibonacci(40));
    let mut fib = Fib::new();
    println!("fibonacci_memo(5): {}", fib.fibonacci_memo(40));
}


pub fn fibonacci(n: i32) -> i32{
    if n == 1 || n == 0 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
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