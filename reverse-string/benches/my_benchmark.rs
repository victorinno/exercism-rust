use criterion::{black_box, criterion_group, criterion_main, Criterion};

use reverse_string::reverse;
// use reverse_string::reverse2;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("reverse", |b| b.iter(|| reverse(black_box("I'm hungry!"))));
    // c.bench_function("reverse2", |b| b.iter(|| reverse2(black_box("I'm hungry!"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
