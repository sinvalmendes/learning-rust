fn main() {
    println!("Hello, world!");
    let x = 5; // immutable variable declaration with type inference
    println!("The value of immutable x is {}", x);

    let mut x = 5; //mutable variable declaration with type inference
    println!("The value of mutable x is {}", x);

    x = 6;
    println!("The value of mutated x is {}", x);

    const Y: u32 =  10; // const declares a constant value, it WILL NOT change!
    println!("The value of constant Y is {}", Y)

}
