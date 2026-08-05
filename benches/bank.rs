use criterion::{Bencher, Criterion, criterion_group, criterion_main};
extern crate rand;
use rand::random_range;
use rust_test::Account;
use rust_test::Bank;

fn bank_benchmark(c: &mut Criterion) {
    c.bench_function("Bank search and transer", |b| {
        b.iter(|| {
            let mut database = Bank::new();
            for i in 0..10000 {
                database.add_customer(Account::new(i, String::from("account #{i}"), 1000.0));
            }

            for i in 0..5000 {
                database.trasfer(random_range(0..=10000), random_range(0..=10000), 700.0);
            }
        })
    });
}

criterion_group!(benches, bank_benchmark);
criterion_main!(benches);

// criterion = "0.4.0"

// [[bench]]
// name = "bank_bench"
// harness = false
