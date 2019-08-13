extern crate rayon;

use rayon::prelude::*;

fn main() {
    println!("Hello, world!");

    let input = vec![1, 2, 3, 4, 5];
    let result = sum_array_iterative(input);
    assert_eq!(15, result);

    let mut par_input = vec![1, 2, 3, 4, 5];
    let result = sum_array_parallel(&mut par_input);
    assert_eq!(15, result);
}

pub fn sum_array_parallel(input: &mut Vec<i32>) -> i32 {
    let result: i32 = input.par_iter_mut().map(|&mut p| p).sum();
    result
}

pub fn sum_array_iterative(input: Vec<u32>) -> u32 {
    let mut counter: u32 = 0;
    for num in input {
        counter += num;
    }
    counter
}
