extern crate rayon;

mod sum;

pub fn sum_array_parallel(input: &mut Vec<i64>) -> i64 {
    let result = sum::sum_array_parallel(input);
    result
}

pub fn sum_vec_iterative(input: &mut Vec<i64>) -> i64 {
    let result = sum::sum_vec_iterative(input);
    result
}

pub fn sum_array_iterative(input: &[i64]) -> i64 {
    let result = sum::sum_array_iterative(input);
    result
}

pub fn rayon_join_sum(mut a: &mut Vec<i64>, mut b: &mut Vec<i64>) -> i64 {
    let mut count_a = 0;
    let mut count_b = 0;
    rayon::join(
        || count_a = sum_vec_iterative(&mut a),
        || count_b = sum_vec_iterative(&mut b),
    );
    return count_a + count_b;
}

pub fn rayon_join_sum2(input: &mut Vec<i64>) -> i64 {
    let size = input.len() / 2;
    let mut iter = input.chunks(size);
    let first_part = iter.next().unwrap();
    let second_part = iter.next().unwrap();
    let mut count_a = 0;
    let mut count_b = 0;
    rayon::join(
        || count_a = sum::sum_array_iterative(first_part),
        || count_b = sum::sum_array_iterative(second_part),
    );
    return count_a + count_b;
}
