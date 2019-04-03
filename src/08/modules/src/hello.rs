
pub fn say_hello(s: &mut String) {
    s.push_str(", Hello!");
    println!("{}", s);
}
