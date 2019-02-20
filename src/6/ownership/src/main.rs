fn main() {
    let s = String::from("Hello, world!");
    println!("{}", s);

    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str is an append function of String type
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("x:{}, y:{}", x, y);

    let a = String::from("a value");
    let b = a; // this moves the value of a to b, and it makes a invalid
    // println!("a:{}", a); // this breaks the compile because a has not his original value anymore, it was moved to b
    println!("b:{}", b); // b has the a value, once it was moved on line 14
    
    let a = String::from("a value");
    let b = a.clone(); // b is a clone of a, so a is still valid, this is a deep copy, and entire copy of the heap value
    println!("a:{} b:{}", a, b);

    // But why this works?
    let x = 5;

    // we are not moving the x value to y, Rust is storing these vlaues in the Stack, the copy there is easy and fast so it does copy x value to y
    let y = x;     
    println!("x:{}, y:{}", x, y);

    let s = String::from("String");
    takes_ownership(s); // s is being borrowed to takes_ownership function
    // println!("{}", s); // this breaks compile, s is not valid here anymore

    let i = 5;
    gets_a_copy(i);
    println!("i is mine! i:{}", i); // i never left this scope, it was copied to the gets_a_copy function

    let s = gives_ownership();
    println!("The s value comes from another function's scope, s:{}", s);

    let s = String::from("s was borrowed and came back");
    let s = takes_ownership_and_gives_back(s);
    println!("I got s back, s:{}", s);
}

fn takes_ownership(s: String) {
    println!("s is mine now haha! s:{}", s);
}

fn gets_a_copy(i: i32) {
    println!("i is a Copy type, the original not mine :(, but hey, I got a copy! i:{}" , i);
}

fn gives_ownership() -> String {
    let s = String::from("gives_ownership string");
    println!("I'm giving ownership of s when I return it");
    s
}

fn takes_ownership_and_gives_back(s: String) -> String {
    println!("I took ownership of s, but I'll give it back s:{}", s);
    s
}