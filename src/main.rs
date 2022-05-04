use aftermath::class_parser::Parser;
use std::thread;
use std::time::{Duration, Instant};
mod class_parser;

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

fn main() {
    let file = std::fs::read("/home/gimbles/Desktop/Aftermath/class_basket/large.class").unwrap();
    let start = Instant::now();
    let mut parser = black_box(Parser::new(black_box(&file)));
    let parsed = black_box(parser.parse().unwrap());
    dbg!(start.elapsed());
    thread::sleep(Duration::from_secs(15));
    dbg!(parsed);
}

#[cfg(tests)]
mod tests;
