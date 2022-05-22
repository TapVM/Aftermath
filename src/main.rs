use aftermath::class_parser::Parser;
use color_eyre::Result;
mod class_parser;

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut parser = Parser::new(include_bytes!(concat!(
        env!("HOME"),
        "/Desktop/Aftermath/class_basket/UsingToStringOrdering.class"
    )));

    dbg!(parser.parse()?);

    Ok(())
}
