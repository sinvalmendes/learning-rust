extern crate rayon;

use rayon::prelude::*;

pub fn sum_array_parallel(input: &mut Vec<i32>) -> i32 {
    let result: i32 = input.par_iter_mut().map(|&mut p| p).sum();
    result
}

pub fn sum_array_iterative(input: &mut Vec<i32>) -> i32 {
    let mut counter: i32 = 0;
    for num in input {
        counter += *num;
    }
    counter
}
