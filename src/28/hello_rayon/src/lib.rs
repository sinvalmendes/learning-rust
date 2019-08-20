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

pub fn join_test(a: &mut Vec<i32>, b: &mut Vec<i32>) -> i32 {
    let result = rayon_sum(a, b);
    result
}

fn rayon_sum(mut a: &mut Vec<i32>, mut b: &mut Vec<i32>) -> i32 {

    let mut count_a = 0;
    let mut count_b = 0;
    rayon::join(
        || count_a = sum_array_iterative(&mut a),
        || count_b = sum_array_iterative(&mut b),
    );

    return count_a + count_b;
}
