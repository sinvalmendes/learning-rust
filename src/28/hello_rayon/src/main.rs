fn main() {
    println!("Hello, world!");

    let input = vec![1, 2, 3, 4, 5];
    let result = sum_array_iterative(input);

    assert_eq!(15, result);
}

fn sum_array_iterative(input: Vec<u32>) -> u32 {
    let mut counter: u32 = 0;
    for num in input {
        counter += num;
    }
    counter
}
