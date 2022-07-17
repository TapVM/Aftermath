#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut parser = aftermath::class_parser::Parser::new(data);
    let _ = parser.parse();
});
