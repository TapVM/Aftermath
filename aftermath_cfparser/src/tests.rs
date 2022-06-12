use crate::class_parser::{self, ParsingError};
use std::io::Cursor;

#[test]
pub fn invalid_magic() {
    let mut parser = class_parser::Parser::new(
        include_bytes!("../class_basket/intentional_invalid_magic.class").to_vec(),
    );
    let parse = parser.parse();

    match parse {
        Ok(_) => {
            panic!(
                "This *should've* returned an error. Specifically the invalid magic error. Somethin\
g's broken."
            )
        }
        Err(e) => match e {
            ParsingError::Magic => {}
            _ => {
                panic!(
                    "While this did return an error, it returned the wrong error. It should've retu\
rned the invalid magic error. Something's broken."
                )
            }
        },
    }
}
