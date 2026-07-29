use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use rust_test::{sort_algo_1, sort_algo_2};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers = vec![
        1, 2, 3, 8, 5, 6, 4, 3, 5, 6, 6, 44, 32, 4354, 34, 556, 443, 434, 546, 5, 43, 3, 4, 435,
        34, 4, 56, 5,
    ];

    c.bench_function("Sorting algorithm", |b: &mut Bencher| {
        b.iter(|| sort_algo_2(&mut numbers));
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
