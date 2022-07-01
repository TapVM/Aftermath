use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod class_parser;
pub mod errors;
pub mod verification;
pub mod consts;
