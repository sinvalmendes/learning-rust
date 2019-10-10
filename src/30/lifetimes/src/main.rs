fn main() {
    let x: i32 = 5;
    let y: i32 = 2;

    let e;
    {
        let w = &y;
        e = helper(&x, w);
    }
    // println!("w: {}", w); // w doesn't exist anymore
    println!("e: {}", e); // but it's value now it's in e

    {
        let i;
        {
            let z: &'static i32 = &3;
            i = helper(&x, z);
            println!("z: {}", z);
        }
        // println!("z: {}", z); // z doesn't exist anymore
        println!("i: {}", i); // but z value is now in i
    }
    // println!("i: {}", i); // i doesn't exist anymore
}

fn helper<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x < y {
        x
    } else {
        y
    }
}
