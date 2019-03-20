fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // ampersands(&) is pointer semantics, we are borrowing the reference
    println!("s:{} len:{}", s, len);

    let mut s = String::from("hello"); // we need to make s mutable in order to pass its mutable reference to other scopes mutate its value
    borrowing_to_mutate(&mut s); // borrowing a mutable reference
    println!("s mutated: {}", s); // it is done, s mutated and is still valid in the current scope!

    let mut counter = 0;
    println!("counter: {}, before mutation", counter);
    increment_mutable_counter_reference(&mut counter);
    println!("counter: {}, after first mutation", counter);
    increment_mutable_counter_reference(&mut counter); // you can borrow the mutable reference as much as you want
    println!("counter: {}, after second mutation", counter);

    let mut counter_to_be_borrowed_twice = 0;
    println!("counter_to_be_borrowed_twice: {}, before mutation", counter_to_be_borrowed_twice);
    borrow_to_another_function(&mut counter_to_be_borrowed_twice);
    println!("counter_to_be_borrowed_twice: {}, after mutation", counter_to_be_borrowed_twice);


}

fn calculate_length(s_pointer: &String) -> usize { // &String means the function want a pointer/reference to a String value in the Heap
    // s_pointer is a pointer (reference) to s (which is also a pointer, but to a Heap value String "hello")
    let len = s_pointer.len(); // we are using s_pointer reference to access s value and its behaviour(len())
    len
}

fn borrowing_to_mutate(s_mutable_pointer: &mut String) {
    s_mutable_pointer.push_str(" world");
}

fn increment_mutable_counter_reference(counter: &mut i32) {
    *counter += 1; // counter is incremented and won't die at the end of this function
}

fn borrow_to_another_function(counter: &mut i32) {
    increment_mutable_counter_reference(counter); // borrowing the already borrowd mutable reference
    // counter won't die at the end of this function because this function doesn't own it, just have it borrowed as mutable
}