use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regex::Regex;

pub fn reply(message: &str) -> &str {
    let treated_message = &message.replace("\t", "");
    let treated_message = treated_message.trim();
    match treated_message {
        m if m.len() == 0 => "Fine. Be that way!",
        m if m.to_uppercase() == m
            && m.get(m.len() - 1..).unwrap() == "?"
            && Regex::new(r"[a-zA-Z]").unwrap().is_match(&m) =>
        {
            "Calm down, I know what I'm doing!"
        }
        m if m.get(m.len() - 1..).unwrap() == "?" => "Sure.",
        m if m.to_uppercase() == m && Regex::new(r"[a-zA-Z]").unwrap().is_match(&m) => {
            "Whoa, chill out!"
        }
        _ => "Whatever.",
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