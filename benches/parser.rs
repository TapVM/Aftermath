use aftermath::class_parser::Parser;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parser_benchmark(c: &mut Criterion) {
    c.bench_function("hello world", |b| {
        b.iter(|| {
            let mut parser = black_box(Parser::new(black_box(
                include_bytes!("../class_basket/hello_world.class").to_vec(),
            )));
            black_box(parser.parse().unwrap());
        })
    });
}
criterion_group!(group, parser_benchmark);
criterion_main!(group);
