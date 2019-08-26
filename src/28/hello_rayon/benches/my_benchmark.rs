#[macro_use]
extern crate criterion;
extern crate hello_rayon;

use criterion::Criterion;
use hello_rayon::sum_array_iterative;
use hello_rayon::sum_array_parallel;
use hello_rayon::rayon_join_sum;

fn create() -> Vec<i64> {
    let mut input: Vec<i64> = vec![];
    for n in 0..1000000 {
        input.push(n);
    }
    return input;
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut input: Vec<i64> = create();
    c.bench_function("sum_array_iterative", move |b| {
        b.iter(|| sum_array_iterative(&mut input))
    });
}

fn criterion_benchmark2(c: &mut Criterion) {
    let mut input: Vec<i64> = create();
    c.bench_function("sum_array_parallel", move |b| {
        b.iter(|| sum_array_parallel(&mut input))
    });
}

fn criterion_benchmark3(c: &mut Criterion) {
    let input: Vec<i64> = create();
    let (first_part, second_part) = input.split_at(input.len() / 2);
    let mut first_part = first_part.to_vec();
    let mut second_part = second_part.to_vec();
    c.bench_function("rayon_join_sum", move |b| b.iter(|| rayon_join_sum(&mut first_part, &mut second_part)));
}

criterion_group!(
    benches,
    criterion_benchmark,
    criterion_benchmark2,
    criterion_benchmark3
);
criterion_main!(benches);
