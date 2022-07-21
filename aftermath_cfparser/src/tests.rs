//------ aftermath_cfparser -- Classfile parser for the Aftermath JVM. -------//
//
// Part of the Aftermath JVM, under the MIT license. The LICENSE file is present
// at the project root, please consult to it for license information.
//
//===----------------------------------------------------------------------===//
//
// This file contains tests for the parser.
//
//===----------------------------------------------------------------------===//

use crate::class_parser::{self, ParsingError};

#[test]
pub fn invalid_magic() {
    let mut parser = class_parser::Parser::new(include_bytes!(
        "../../class_basket/intentional_invalid_magic.class"
    ));

    let parse = parser.parse();

    match parse {
        Ok(_) => panic!("Test shouldn't have passed, Expected an Magic error."),
        Err(e) => {
            if !matches!(e, ParsingError::Magic) {
                panic!(
                    "Test shouldn't have failed with this error.
                Expected error -> Magic
                Found error -> {:?}",
                    e
                );
            }
        }
    }
}

#[test]
pub fn issue_9_out_of_bound_indexing() {
    let mut parser = class_parser::Parser::new(&[]);
    let parse = parser.parse();
}

#[test]
pub fn issue_10_integer_underflow_while_subtracting() {
    #[rustfmt::skip]
    let mut parser = class_parser::Parser::new(
        &[202, 254, 186, 190, 255, 255, 255, 1, 0, 0]
    );

    let _ = parser.parse();
}
