mod hello;
mod submodule;
use submodule::module::say_hello_submodule;
fn main() {
    let mut s = String::from("John");
    hello::say_hello(&mut s);
    say_hello_submodule();
}
