fn main() {
    println!("Hello, world!");

    let x = 5; // x is a pointer to a i32 value which is 5 (stored on the Stack)
    let y = &x;

    println!("x: {}", x); // x points to 5, so the output will be x: 5
    println!("y: {}", y); // y points to x (which points to 5), the println! deref and the output will be x: 5

    assert_eq!(5, x); // True
    // assert_eq!(5, y); // this is different, because y value is acctually a &{integer} (reference to a reference to an integer)
    assert_eq!(5, *y); // True, we are dereferencing y, so we get the value at the and of the "rabbit hole" of references


    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    // this is the last line of the main() function, c and d will go out of scope and our custom drop function will be called
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
