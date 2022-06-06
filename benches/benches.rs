use md5::lib::{convert_and_pad, md5_hash};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(|byte| byte as char)
        .collect()
}

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

fn md5_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("md5");
    for size in (0..=2000).step_by(100) {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_with_setup(|| random_string(size), |s| md5_hash(convert_and_pad(&s)));
        });
    }
    group.finish();
}

criterion_group!(benches, md5_bench);
criterion_main!(benches);
