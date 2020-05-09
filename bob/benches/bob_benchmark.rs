use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn is_yelling(message: &str) -> bool {
    let have_letters: bool = message.chars().filter(|x| x.is_alphabetic()).count() > 0;
    message.to_uppercase() == message && have_letters
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.trim().len() == 0 => "Fine. Be that way!",
        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever."
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("1", |b| b.iter(|| reply(black_box("Okay if like my  spacebar  quite a bit?   "))));
    c.bench_function("2", |b| b.iter(|| reply(black_box("1, 2, 3 GO!"))));
    c.bench_function("3", |b| b.iter(|| reply(black_box("4?"))));
    c.bench_function("4", |b| b.iter(|| reply(black_box("1, 2, 3"))));
    c.bench_function("5", |b| b.iter(|| reply(black_box("Ending with ? means a question."))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);