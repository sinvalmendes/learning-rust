#[macro_use]
extern crate criterion;
extern crate hello_rayon;

use criterion::Criterion;
use hello_rayon::join_test;
use hello_rayon::sum_array_iterative;
use hello_rayon::sum_array_parallel;

fn create() -> Vec<i32> {
    let mut input: Vec<i32> = vec![];
    for n in 0..10000000 {
        input.push(n);
    }
    return input;
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut input: Vec<i32> = create();
    c.bench_function("sum_array_iterative", move |b| {
        b.iter(|| sum_array_iterative(&mut input))
    });
}

fn criterion_benchmark2(c: &mut Criterion) {
    let mut input: Vec<i32> = create();
    c.bench_function("sum_array_parallel", move |b| {
        b.iter(|| sum_array_parallel(&mut input))
    });
}

fn criterion_benchmark3(c: &mut Criterion) {
    // let mut input: Vec<i32> = create();
    let mut input = vec![1, 2, 3, 4, 5, 6];
    c.bench_function("join_test", move |b| b.iter(|| join_test(&mut input)));
}

// criterion_group!(benches, criterion_benchmark, criterion_benchmark2);
criterion_group!(benches, criterion_benchmark3);
criterion_main!(benches);
