use std::{borrow::Cow, fmt::Debug};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Error (Metadata) -> The magic of the class file should only be 0xCAFEBABE! ☕ 💃 ✨")]
    Magic,
    #[error("Error (Metadata) -> If the major version is 56 or above, the minor version can only be 0 or 65535.")]
    MinorVersion,
    #[error("Error (Metadata) -> Major version should be in range 45 to 61")]
    MajorVersion,
    #[error("Error (Constant Pool) -> Invalid constant pool tag -- {0}.")]
    ConstantPoolTag(u8),

    #[error("Error (Access flags and Metadata) -> If the class file is a module, the major_version and minor_version should be ≥ 53.0 (i.e. Java SE 9 and above)")]
    InvalidVersionForModule,

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

    #[error("Error (Class file) -> The field {0} provided an invalid index -- {1} when the length of the array was {2}.")]
    InvalidIndexProvidedByField(Cow<'static, str>, u16, u16),

    #[error("Error (Class file) -> The field {0} provided an invalid index -- {1}, since it expected the {2} type rather it got the {3} type.")]
    InvalidIndexType(Cow<'static, str>, u16, Cow<'static, str>, String),

    #[error("I/O Error -> ")]
    IoError(#[from] std::io::Error),
}
