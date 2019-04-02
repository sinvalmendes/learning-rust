fn main() {
    let closure = |a| { // defining a closure called "closure" that receives parameter "a"
        println!("{}", a);
    };

    let b = String::from("this is b");
    closure(b);

    call_function(callback, "this is the value of 'text' parameter comming from main calling 'call_function' passing 'callback' as a parameter");
}

fn call_function(f: fn(&str), f_param: &str) {
    f(f_param);
}

fn callback(text: &str) {
    println!("this is advanced function usage! {}", text);
}


