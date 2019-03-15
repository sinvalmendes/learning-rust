fn main() {
    println!("Hello, world!");

    let x = 5; // x is a pointer to a i32 value which is 5 (stored on the Stack)
    let y = &x;

    println!("x: {}", x); // x points to 5, so the output will be x: 5
    println!("y: {}", y); // y points to x (which points to 5), the println! deref and the output will be x: 5

    assert_eq!(5, x); // True
    // assert_eq!(5, y); // this is different, because y value is acctually a &{integer} (reference to a reference to an integer)
    assert_eq!(5, *y); // True, we are dereferencing y, so we get the value at the and of the "rabbit hole" of references
}
