extern crate rayon;

mod sum;

pub fn sum_array_parallel(input: &mut Vec<i32>) -> i32 {
    let result = sum::sum_array_parallel(input);
    result
}

pub fn sum_array_iterative(input: &mut Vec<i32>) -> i32 {
    let result = sum::sum_array_iterative(input);
    result
}
