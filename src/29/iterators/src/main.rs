// Run `cargo run` to execute the basic example
// Run `cargo test` to execute a different example

fn main() {
    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
    let v_cloned: Vec<i32> = a.iter().cloned().collect();
    // let v_copied: Vec<_> = a.iter().copied().collect(); // works only from rustc 1.36.0+
    let v_mapped_copy: Vec<i32> = a.iter().map(|&x| x).collect();

    assert_eq!(vec![2, 4, 6], doubled);
    assert_eq!(vec![1, 2, 3], v_cloned);
    assert_eq!(vec![1, 2, 3], v_mapped_copy);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
pub fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
