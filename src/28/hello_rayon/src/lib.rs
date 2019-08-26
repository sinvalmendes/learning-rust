extern crate rayon;

mod sum;

pub fn sum_array_parallel(input: &mut Vec<i64>) -> i64 {
    let result = sum::sum_array_parallel(input);
    result
}

pub fn sum_array_iterative(input: &mut Vec<i64>) -> i64 {
    let result = sum::sum_array_iterative(input);
    result
}

pub fn rayon_join_sum(mut a: &mut Vec<i64>, mut b: &mut Vec<i64>) -> i64 {
    let mut count_a = 0;
    let mut count_b = 0;
    rayon::join(
        || count_a = sum_array_iterative(&mut a),
        || count_b = sum_array_iterative(&mut b),
    );
    return count_a + count_b;
}
