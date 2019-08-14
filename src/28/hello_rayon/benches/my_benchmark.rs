#[macro_use]
extern crate criterion;
extern crate hello_rayon;

use criterion::Criterion;
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

criterion_group!(benches, criterion_benchmark, criterion_benchmark2);
criterion_main!(benches);
