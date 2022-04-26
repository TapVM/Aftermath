use aftermath::class_parser::Parser;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Cursor;

fn parser(c: &mut Criterion) {
    c.bench_function("LARGE class", |b| b.iter(|| {}));
}

criterion_group!(group, parser);
criterion_main!(group);
