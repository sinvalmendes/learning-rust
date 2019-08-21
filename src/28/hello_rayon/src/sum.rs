extern crate rayon;

use rayon::prelude::*;

pub fn sum_array_parallel(input: &mut Vec<i64>) -> i64 {
    let result: i64 = input.par_iter_mut().map(|&mut p| p).sum();
    result
}

pub fn sum_array_iterative(input: &mut Vec<i64>) -> i64 {
    let mut counter: i64 = 0;
    for num in input {
        counter += *num;
    }
    counter
}
