use color_eyre::Result;
mod class_parser;
use class_parser::verification::Verifier;
use class_parser::Parser;

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

    let file = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/class_basket/UsingToStringOrdering.class"
    ));

    let mut parser = Parser::new(file);
    let parsed = parser.parse()?;

    let verification = Verifier::new(parsed);
    dbg!(verification.verify()?);

    Ok(())
}
