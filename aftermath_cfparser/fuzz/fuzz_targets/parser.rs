//------ aftermath_cfparser -- Classfile parser for the Aftermath JVM. -------//
//
// Part of the Aftermath JVM, under the MIT license. The LICENSE file is present
// at the project root, please consult to it for license information.
//
//===----------------------------------------------------------------------===//
//
// This file contains a fuzzer for the parser. This was used to fix
// - #9 (Fuzz: Out of bound indexing errors)
// - #10 (Fuzz: Integer underflow from subtracting)
//
//===----------------------------------------------------------------------===//

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut parser = aftermath::class_parser::Parser::new(data);
    let _ = parser.parse();
});
