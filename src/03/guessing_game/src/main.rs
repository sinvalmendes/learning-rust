use std::io; // this is an import of package std.io (:: == .)
use std::cmp::Ordering;
use rand::Rng; // we are not using Rng trait but we need this scope to access thread_rng method


fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1, 100); // the : indicates u32 is the type of random_number 
    
    println!("I want to play a game... guess the number!");
    loop {
        let mut guess = String::new(); // this is how you declare a VARIABLE (mut == mutable)
        println!("Please input your guess:");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!"); // when using the call with "." syntax we break lines
        println!("{}",guess);
        let guess: u32 = match guess.trim().parse() { // trim() method eliminates \n and spaces
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }, 
        };
        println!("You guessed {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Your guess is lower!"),
            Ordering::Equal => { // when the patter match with Equal we execute two instructions
                println!("Your nail it!"); 
                break;
            }, 
            Ordering::Greater => println!("Your guess is greater!"),
        }
    }
    println!("The random number is {}", random_number);



    let immutable = 5; // immutable variable (constant?)
    let mut mutable = 5; // mutable variable (this one we can change the values)

    println!("immutable {} - mutable {}", immutable, mutable);
    mutable = 6;
    println!("immutable {} - mutable (mutated) {}", immutable, mutable);
}
