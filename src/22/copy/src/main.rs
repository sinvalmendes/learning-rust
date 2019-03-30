#[derive(Debug)]
struct NormalFoo {
    id: i32,
}

#[derive(Debug, Copy, Clone)] // CopyFoo is just like NormalFoo but it implements Copy and Clone traits, so it will be able to use Copy Semantics
struct CopyFoo {
    id: i32,
}

#[derive(Debug, Copy, Clone)]
struct AnotherCopyFoo {
    name: &'static str, // name is a string literal type, and string literal implements Copy trait just like other primitive types like i32 for e.g
}

// #[derive(Debug, Copy, Clone)] // this will lead to an E0204 error, because the String, type of the name field, does not implement Copy, thus StringCopyFoo cannot be Copy
// struct StringCopyFoo {
//     name: String,
// }

fn main() {
    let a = NormalFoo { id: 1 };
    println!("{:?}", a);

    let b = a;
    // println!("{:?}", a); // this doesn't compile because a value has moved to b, a is not valid anymore (Move Semantics)
    println!("{:?}", b); // b points to NormalFoo {name: "a"}

    let c = CopyFoo { id: 2 };
    println!("{:?}", c);

    let d = c;
    println!("{:?}", c); // this will compile because c is valid here, its value was not MOVED to d, it was COPIED because CopyFoo implements Copy and Clone!
    println!("{:?}", d);

    let e = AnotherCopyFoo { name: "e" }; //this will compile and work because e implements Copye and Clone traits
    println!("{:?}", e);

    let f = e;
    println!("{:?}", e); // e is still valid, because f has a copied value of e
    println!("{:?}", f);
}

// CopyFoo only works as a Copy trait because it's fields (id: i32) are able to implement Copy!
// Types like String, str, Vec and many others are not able to implement Copy in a Rustacean/Safe manner!
// Thus, if your type T has one of more fields that does not implement the Copy trait, your type T won't be able as well.
