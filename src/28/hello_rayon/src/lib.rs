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

pub fn join_test(input: &mut Vec<i32>) -> i32 {
    let result = rayon_sum(input);
    result
}

fn rayon_sum(v: &mut Vec<i32>) -> i32 {
    let (a, b) = v.split_at(v.len() / 2);
    let mut a = a.to_vec();
    let mut b = b.to_vec();

    let mut count_a = 0;
    let mut count_b = 0;
    rayon::join(
        || count_a = sum_array_iterative(&mut a),
        || count_b = sum_array_iterative(&mut b),
    );

    return count_a + count_b;
}
