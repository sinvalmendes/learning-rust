#[derive(Debug)] // tells the compiler to use a default formating of this Struct, since it's not implementing the string format function
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("{:?}", rect1); // tells the compiler to print rect1 using an default formating 
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is calculated with a method, it is {} square pixels.",
        rect1.area()
    );}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}