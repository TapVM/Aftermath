#![allow(dead_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    // Parsing errors.
    #[error(
        "Malformed class -> The magic of the class file should only be 0xCAFEBABE! ☕ 💃 ✨
        Help ~> Are you *sure* that this is a JVM class file?"
    )]
    Magic,

    #[error("Malformed class -> Invalid constant pool tag -> {0}.")]
    ConstantPoolTag(u8),

    #[error("Malformed class -> Attribute index is not a UTF8 node in the constant pool")]
    AttributeNotUtf8,

    #[error("Malformed class -> Invalid Element Value -> `{0}`")]
    InvalidElementValue(char),

    #[error("Malformed class -> Invalid target type -> `{0}`")]
    InvalidTargetType(u8),

    #[error("Malformed class -> Invalid frame type -> `{0}`")]
    InvalidFrameType(u8),

    #[error("Malformed class -> Invalid verification type info tag -> `{0}`")]
    InvalidTagVerificationTypeInfo(u8),

    #[error("Malformed class -> Invalid Attribute -> `{0}`")]
    InvalidAttribute(String),

    #[error("I/O Error -> ")]
    IoError(#[from] std::io::Error),
    // ---------------------------------------------------------------------------------------------
    // Verification errors
    #[error("Malformed class -> Invalid Major version")]
    InvalidMajorV,
}
