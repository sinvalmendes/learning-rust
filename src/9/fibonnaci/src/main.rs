use std::io;

fn fib(n: i32) -> i32 {
    if n == 1 || n == 0 {
        return 1;
    }
    return fib(n-1) + fib(n-2);
}

fn main() {
    loop {
        println!("Please input your number:");
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line!");
        let n: i32 = match n.trim().parse() { // trim() method eliminates \n and spaces
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }, 
        };

        
        println!("fib({}): {}", n, fib(n));
    }
}
