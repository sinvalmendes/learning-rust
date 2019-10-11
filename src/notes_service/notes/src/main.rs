fn main() {
    println!("Hello, world!");
    let system = actix::System::new("test");

    system.run();
}
