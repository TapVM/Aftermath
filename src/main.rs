use aftermath::class_parser::Parser;
use color_eyre::Result;
use std::ffi::OsStr;
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

fn main() -> Result<()> {
    color_eyre::install()?;

    std::fs::read_dir("./class_basket")?
        .map(|x| x.as_ref().unwrap().path())
        .filter(|x| {
            x.extension() == Some(OsStr::new("class"))
                && dbg!(x.file_name()) != Some(OsStr::new("intentional_invalid_magic.class"))
        })
        .for_each(|x| {
            let data = std::fs::read(x).unwrap();
            let mut parser = Parser::new(&data);
            parser.parse().unwrap();
        });

    Ok(())
}

#[cfg(tests)]
mod tests;
