fn main() {
    let mut x = "value";
    println!("x:{}", x);
    x = "mutated";
    println!("x after mutation: {}", x);

    let y = &mut x;
    println!("y: {}", y);
    // let z = &mut x; // this won't compile, there can only be one borrowed value, and right now y is the only one

    *y = "mutated by y";
    println!("y after mutation: {}", y);
    println!("x after mutation: {}", x);

    x = "mutated by x";
    println!("x after mutation: {}", x);
    // y is a borrow, and cannot be used anymore, x is the only valid reference now
    // there can only be one mutable borrowed reference to a value, but y can borrow x again...
}
