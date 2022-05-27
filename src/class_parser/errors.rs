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
    #[error("Malformed class -> Invalid Minor version")]
    InvalidMinorV,
    #[error("Malformed class -> The interface flag was set without the abstract flag being set, which is invalid.")]
    InterfaceWithoutAbstract,
    #[error("Malformed class -> The interface flag was set, while one (or all) of the following flags was set, which is illegal.
    • Final
    • Super
    • Enum
    • Module")]
    IllegalFlagsWhileInterface,
    #[error("Malformed class -> The interface flag was not set, while one (or both) of the following flags was set, which is illegal.
    • Annotation
    • Module")]
    IllegalFlagsWhileNonInterface,
    #[error("Malformed class -> The interface flag was not set, while both of the following flags was set, which is illegal.
    • Final
    • Abstract")]
    FinalAndAbstractWhileNonInterface,
    #[error("Malformed class -> The annotation flag was set, but the interface flag was not set, which is illegal.")]
    AnnotationWithoutInterface,
    #[error("Malformed class -> Invalid version as Module, major version must be equal or bigger than 53.")]
    InvalidVersionAsModule,
    #[error("Malformed class -> Binary Class or Interface name contains the `.` (Dot/Period) character, which is illegal.")]
    BinaryNameContainsDot,
    #[error("Malformed class -> A Class constant pool node did not point to a Utf8 node in the constant pool, which is illegal")]
    ClassNodeNotPointingToUtf8,
    #[error("Malformed class -> this_class was NOT module-info, but the class file is a module, which is illegal.")]
    ThisClassNotModuleInfoAsModule,
    #[error("Malformed class -> The Module flag was set, but one (or all) of the following fields were not zero, which is illegal.
    • super_class
    • interfaces_count
    • fields_count
    • methods_count")]
    VarsNotZeroAsModule,
    #[error("Malformed class -> A FieldRef constant pool node's `class_index` field did not point to a Class constant pool node, which is illegal.")]
    FieldRefNodeNotPointingToClass,
    #[error("Malformed class -> A FieldRef constant pool node's `name_and_type` field did not point to a NameAndType constant pool node, which is illegal.")]
    FieldRefNodeNotPointingToNameAndType,
    #[error("Malformed class -> A MethodRef constant pool node's `class_index` field did not point to a Class constant pool node, which is illegal.")]
    MethodRefNodeNotPointingToClass,
    #[error("Malformed class -> A MethodRef constant pool node's `name_and_type` field did not point to a NameAndType constant pool node, which is illegal.")]
    MethodRefNodeNotPointingToNameAndType,
    #[error("Malformed class -> A InterfaceMethodRef constant pool node's `class_index` field did not point to a Class constant pool node, which is illegal.")]
    InterfaceMethodRefNodeNotPointingToClass,
    #[error("Malformed class -> A InterfaceMethodRef constant pool node's `name_and_type` field did not point to a NameAndType constant pool node, which is illegal.")]
    InterfaceMethodRefNodeNotPointingToNameAndType,
    #[error("Malformed class -> A String constant pool node did not point to a Utf8 constant pool node, which is illegal.")]
    StringNodeNotPointingToUtf8,
    #[error("Malformed class -> A MethodType constant pool node did not point to a Utf8 constant pool node, which is illegal.")]
    MethodTypeNodeNotPointingToUtf8,
    #[error("Malformed class -> A Module constant pool node did not point to a Utf8 constant pool node, which is illegal.")]
    ModuleNodeNotPointingToUtf8,
    #[error("Malformed class -> A Package constant pool node did not point to a Utf8 constant pool node, which is illegal.")]
    PackageNodeNotPointingToUtf8,
    #[error("Malformed class -> The class has one (or both) of the Dynamic and InvokeDynamic attributes, but it doesn't contain a valid amount of BootstrapMethods (There must only be 1), which is illegal.")]
    InvalidAmountOfBootStrapMethodsInClass,
    #[error("Malformed class -> The bootstrap_method_attr_index given by a Dynamic constant pool node was not a valid index into bootstrap_methods, which is illegal.")]
    BootstrapMethodAttrIndexInDynamicAttributeIsNotValidIndex,
    #[error("Malformed class -> An InvokeDynamic constant pool node's `name_and_type_index` did not point to a NameAndType node in the constant pool, which is illegal.")]
    InvokeDynamicNotPointingToNameAndType,
    #[error("Malformed class -> A Dynamic constant pool node's `name_and_type_index` did not point to a NameAndType node in the constant pool, which is illegal.")]
    DynamicNotPointingToNameAndType,
    #[error("Malformed class -> Malformed class -> The bootstrap_method_attr_index given by an InvokeDynamic constant pool node was not a valid index into bootstrap_methods, which is illegal.")]
    BootstrapMethodAttrIndexInInvokeDynamicAttributeIsNotValidIndex,
    #[error("Malformed class -> The value of reference_kind in a MethodHandle constant pool node wasn't in range (1..=9), which is illegal.")]
    InvalidReferenceKind,
    #[error("Malformed class -> The value of reference_kind in a MethodHandle constant pool node was in the range 1..=4, but the reference_index didn't point to a FieldRef, which is illegal.")]
    MethodHandle1to4NotPointingToFieldRef,
    #[error("Malformed class -> The value of reference_kind in a MethodHandle constant pool node was either 5 or 8, but the reference_index didn't point to a MethodRef, which is illegal.")]
    MethodHandle5or8NotPointingToMethodRef,
}
