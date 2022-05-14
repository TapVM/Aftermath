#![allow(dead_code)]

use std::str::Utf8Error;

use thiserror::Error;

use super::{U2, U4};

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error(
        "Error (Metadata) -> The magic of the class file should only be 0xCAFEBABE! ☕ 💃 ✨
        Help ~> Are you *sure* that this is a JVM class file?"
    )]
    Magic,
    #[error("Error (Metadata) -> If the major version is 56 or above, the minor version can only be 0 or 65535.")]
    MinorVersion,
    #[error("Error (Metadata) -> Major version should be in range 45 to 61")]
    MajorVersion,
    #[error("Error (Constant Pool) -> Invalid constant pool tag -- {0}.")]
    ConstantPoolTag(u8),

    #[error("Error (Access flags) -> If the ACCESS_MODULE flag is set, no other flag is allowed to be set.")]
    ContainsOtherFlagsWhileBeingAModule,

    #[error("Error (Access flags and Metadata) -> If the class file is a module, the major_version and minor_version should be ≥ 53.0 (i.e. Java SE 9 and above)")]
    InvalidVersionForModule,

    #[error("Error (Access flags) -> This class file is a module, but one of (or more) of the following variables
    • Super class
    • Interfaces count
    • Fields count
    • Methods count

    were not zero, which is illegal.")]
    ModuleHasIllegalVariables,

    #[error("Error (Access flags) -> The ACCESS_INTERFACE flag was set without the ACCESS_ABSTRACT flag, which is illegal.")]
    InterfaceWithoutAbstract,

    #[error("Error (Access flags) -> The ACCESS_INTERFACE flag was set while one (or all) of the following flags was set.
    • ACCESS_FINAL
    • ACCESS_SUPER
    • ACCESS_ENUM
    • ACCESS_MODULE

    which is illegal.")]
    ContainsIllegalFlagsAsInterface,

    #[error("Error (Access flags) -> The ACCESS_INTERFACE flag was NOT set while one (or all) of the following flags was set.
    • ACCESS_ANNOTATIONS
    • ACCESS_MODULE

    which is illegal.")]
    ContainsIllegalFlagsAsNonInterface,

    #[error("Error (Access flags) -> The ACCESS_INTERFACE flag was NOT set while BOTH of these flags was set.
    • ACCESS_FINAL
    • ACCESS_ABSTRACT

    which is illegal.")]
    ContainsFinalAndAbstractAsNonInterface,

    #[error("Error (Access flags) -> The ACCESS_ANNOTATION flag was set while the ACCESS_INTERFACE was NOT set, which is illegal.")]
    AnnotationWithoutInterface,

    #[error("Error (Class file) -> The field {0} provided an illegal index -- {1} when the length of the array was {2}.")]
    InvalidIndexProvidedByField(String, u16, u16),

    #[error("Error (Class file) -> The field {0} provided an illegal index -- {1}, since it expected the {2} type rather it got the {3} type.")]
    InvalidIndexType(String, u16, String, String),

    #[error(
        "Error (Attributes) -> An attribute gave an invalid index ({0:?}) to the constant pool."
    )]
    AttributeIndex(U2),

    #[error("Error (Attributes) -> An attribute index returned a constant pool node which was NOT Constant_Utf8")]
    AttributeNotUtf8,

    #[error("Error (Utf8) -> UTF8 node in constant pool is NOT valid UTF8")]
    Utf8Error(#[from] Utf8Error),

    #[error("Error (Attributes) -> The attribute length should've been {0}, but got {1}")]
    AttributeLength(U4, U4),

    #[error("Error (Attributes) -> The Value attribute can only have the Integer, Float, Long, Double or String constant pool index.")]
    IllegalValueAttribute,

    #[error("I/O Error -> ")]
    IoError(#[from] std::io::Error),
}
