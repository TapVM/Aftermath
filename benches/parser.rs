use aftermath::class_parser::Parser;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parser(c: &mut Criterion)
{
    let x = std::fs::read("./Benchmark.class").unwrap();
    c.bench_function("LARGE class", |b| {
        b.iter(|| {
            let mut parser = black_box(Parser::new(black_box(&x)));
            black_box(parser.parse().unwrap());
        })
    });
}

criterion_group!(group, parser);
criterion_main!(group);
