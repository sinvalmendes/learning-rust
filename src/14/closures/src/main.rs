fn main() {
    let closure = |a| { // defining a closure called "closure" that receives parameter "a"
        println!("{}", a);
    };

    let b = String::from("this is b");
    closure(b);
}
