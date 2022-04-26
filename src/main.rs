pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
mod class_parser;
use class_parser::Parser;
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let mut parser = black_box(Parser::new(black_box(
        include_bytes!("../class_basket/large.class").to_vec(),
    )));
    black_box(parser.parse().unwrap());
    dbg!(now.elapsed());
}
