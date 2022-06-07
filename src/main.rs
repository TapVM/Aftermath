use color_eyre::Result;
mod class_parser;
use class_parser::verification::Verifier;
use class_parser::Parser;
use std::time::Instant;
pub fn black_box<T>(dummy: T) -> T
{
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

fn main() -> Result<()>
{
    color_eyre::install()?;

    let file = black_box(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/Benchmark.class"
    )));

    for i in 0..10 {
        let now = Instant::now();
        let mut parser = black_box(Parser::new(black_box(file)));
        let parsed = black_box(parser.parse()?);
        dbg!(now.elapsed());
    }

    Ok(())
}
