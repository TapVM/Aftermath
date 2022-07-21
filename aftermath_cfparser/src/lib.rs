//------ aftermath_cfparser -- Classfile parser for the Aftermath JVM. -------//
//
// Part of the Aftermath JVM, under the MIT license. The LICENSE file is present
// at the project root, please consult to it for license information.
//
//===----------------------------------------------------------------------===//
//
// This file re-exports modules for parsing class files.
//
//===----------------------------------------------------------------------===//

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod class_parser;
pub mod consts;
pub mod errors;
pub mod verification;

#[cfg(test)]
mod tests;
