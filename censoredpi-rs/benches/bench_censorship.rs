use censoredpi::{write_censored_digits_of_pi_inplace, write_censored_digits_of_pi_iterative};
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use futures::executor::block_on;

criterion_group!(benches, bench_censorship);
criterion_main!(benches);

fn bench_censorship(c: &mut Criterion) {
    const PI_50K: &str = include_str!("../../pi50k.txt");

    let mut group = c.benchmark_group("censor");

    group.bench_function("inplace", |b| {
        b.iter(|| {
            _ = block_on(write_censored_digits_of_pi_inplace(black_box(PI_50K), futures::io::sink()));
        })
    });

    group.bench_function("iterative", |b| {
        b.iter(|| {
            _ = block_on(write_censored_digits_of_pi_iterative(black_box(PI_50K), futures::io::sink()));
        })
    });

    group.finish();
}
