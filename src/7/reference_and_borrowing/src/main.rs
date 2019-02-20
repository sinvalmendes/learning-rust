fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // ampersands(&) is pointer semantics, we are borrowing the reference
    println!("s:{} len:{}", s, len);

    let mut s = String::from("hello"); // we need to make s mutable in order to pass its mutable reference to other scopes mutate its value
    borrowing_to_mutate(&mut s); // borrowing a mutable reference
    println!("s mutated: {}", s); // it is done, s mutated and is still valid in the current scope!
}

fn calculate_length(s_pointer: &String) -> usize { // &String means the function want a pointer/reference to a String value in the Heap
    // s_pointer is a pointer (reference) to s (which is also a pointer, but to a Heap value String "hello")
    let len = s_pointer.len(); // we are using s_pointer reference to access s value and its behaviour(len())
    len
}

fn borrowing_to_mutate(s_mutable_pointer: &mut String) {
    s_mutable_pointer.push_str(" world");
}