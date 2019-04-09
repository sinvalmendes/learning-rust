use std::rc::Rc;

#[derive(Debug)]
struct bla {}

impl bla {
    fn new() -> bla {
        bla {}
    }
}

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
    // there can only be one mutable borrowed reference to a value at a given time, but y can borrow x again...

    //============================================================
    let mut x = "value";
    println!("x:{}", x);
    x = "mutated";
    println!("x after mutation: {}", x);

    {
        let y = &mut x;
        println!("y: {}", y);

        *y = "mutated by y";
        println!("y after mutation: {}", y);
        println!("x after mutation: {}", x);
    }
    // y is not valid here because it went out of scope on the end of the last closure
    println!("x: {}", x);

    // ============================================================
    let five = 5;
    let a = &five;
    let b = &five;
    println!("a:{} b:{}", a, b); // both a and b points to the value 5 now

    let mut five = Rc::new(5); // Using Rc to encapsulate the ownership of the value 5
    let a = Rc::new(Rc::clone(&mut five)); // Using Rc to create a refrence to five
    let b = Rc::new(Rc::clone(&mut five)); // Using Rc to create another reference to five
    println!("a:{} b:{}", a, b); // both a and b points to the value 5 now

    let bla = bla::new();
    println!("{:?}", bla);
    drop(bla); // bla is dropped here
               // println!("{:?}", bla); // this won't compile, bla was dropped, it does not exist anymore

    let six = 6;
    drop(six); // this does not drop six, because primitive types does not implement the Drop trait
    println!("{}", six); // this works
}
