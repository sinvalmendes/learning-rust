fn main() {
    let clojure = |a| { // defining a clojure called "clojure" that receives parameter "a"
        println!("{}", a);
    };

    let b = String::from("this is b");
    clojure(b);
}
