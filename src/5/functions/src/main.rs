fn main() {
    another_function();

    let mut x = 1;
    let _y = {
        println!("Inside another scope!");
        x = x + 1;
    };
    println!("x was incremented inside another scope, x is {}", x);

    x = returns_value();
    println!("x was assigned to a function return value, x is {}", x);

    x = plus_one(x);
    println!("x was incremented by other function, x is {}", x);

    let array: [i32;7] = [0,1,2,3,4,5,-1];
    print_array_items(&array);
}

fn another_function() {
    println!("Hello functions!");
}

fn returns_value() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_array_items(array: &[i32]) {
    for item in array.iter(){
        println!("{}", item);
    }
}