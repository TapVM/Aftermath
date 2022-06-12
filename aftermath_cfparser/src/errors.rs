#![allow(dead_code)]

use thiserror::Error;

#[derive(Debug)]
pub enum CpNodeError {
    Class,
    String,
    MethodType,
    Module,
    Package,
    Integer,
    Float,
    Dynamic,
    NameAndType,
    InvokeDynamic,
    FieldRef,
    MethodRef,
    InterfaceMethodRef,
    Long,
    Double,
    MethodHandle,
    Utf8,
    None,
}

#[derive(Debug)]
pub enum Attributes {
    Value,
    Code,
    StackMapTable,
    Exceptions,
    InnerClass,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExt,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
}

#[derive(Error, Debug)]
pub enum ParsingError<'a> {
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
    #[error(
        "Malformed class -> Binary Class or Interface name contains the `.` (Dot/Period) character.

    • Help for developers targetting the JVM -> Try replacing the dots with `/` (Slash)."
    )]
    BinaryNameContainsDot,
    #[error("Malformed class -> this_class was NOT \"module-info\", but the class file is a module, which is illegal.")]
    ThisClassNotModuleInfoAsModule,
    #[error("Malformed class -> The Module flag was set, but one (or all) of the following fields were not zero, which is illegal.
    • super_class
    • interfaces_count
    • fields_count
    • methods_count")]
    VarsNotZeroAsModule,

    #[error("Malformed class -> The class has one (or both) of the Dynamic and InvokeDynamic attributes, but it doesn't contain a valid amount of BootstrapMethods (There must only be 1), which is illegal.")]
    InvalidAmountOfBootStrapMethodsInClass,
    #[error("Malformed class -> The bootstrap_method_attr_index given by a Dynamic constant pool node was not a valid index into bootstrap_methods, which is illegal.")]
    BootstrapMethodAttrIndexInDynamicAttributeIsNotValidIndex,
    #[error("Malformed class -> The bootstrap_method_attr_index given by an InvokeDynamic constant pool node was not a valid index into bootstrap_methods, which is illegal.")]
    BootstrapMethodAttrIndexInInvokeDynamicAttributeIsNotValidIndex,

    #[error("Malformed class -> The value of reference_kind in a MethodHandle constant pool node wasn't in range (1..=9), which is illegal.")]
    InvalidReferenceKind,
    #[error("Malformed class -> The value of reference_kind in a MethodHandle constant pool node was in the range 1..=4, but the reference_index didn't point to a FieldRef, which is illegal.")]
    MethodHandle1to4NotPointingToFieldRef,
    #[error("Malformed class -> The value of reference_kind in a MethodHandle constant pool node was either 5 or 8, but the reference_index didn't point to a MethodRef, which is illegal.")]
    MethodHandle5or8NotPointingToMethodRef,

    #[error("Malformed class -> A {0:?} node in the constant pool did not point to a {1:?} node in the constant pool in the {2} field, which is illegal.")]
    InvalidIndexFromNodeToNode(CpNodeError, CpNodeError, &'a str),
    #[error("Malformed class -> A {0:?} attribute in the constant pool did not point to a {1:?} node in the constant pool in the {2} field, which is illegal.")]
    InvalidIndexFromAttributeToNode(Attributes, CpNodeError, &'a str),
    #[error("Malformed class -> A {0:?} attribute in the constant pool did not point to any of {1:?} nodes in the constant pool in the {2} field, which is illegal.")]
    InvalidIndexFromAttributeToNodes(Attributes, &'a [CpNodeError], &'a str),

    #[error("Malformed class -> This class file is a module, but it either
    • Did not have a Module attribute
    • Contained attributes aside Module, ModulePackages, ModuleMainClass, InnerClass, SourceFile, SourceDebugExt, RuntimeVisibleAnnotations, RuntimeInvisibleAnnotations")]
    InvalidAttributesAsModule,

    #[error("Malformed class -> The Code attribute in the class had an invalid code_length, it shouldn't be larger than 65536 or equal to 0.")]
    CodeAttributeCodeLength,
    #[error("Malformed class -> The Code attribute in the class had invalid values of either (or all) of the following fields, they must be valid indexes into the code array, but they weren't.
    • start_pc
    • end_pc
    • handler_pc
    ")]
    InvalidIndexesInCodeAttribute,

    #[error(
        "Malformed class -> The Class file had attributes which weren't from the following list
    • SourceFile
    • InnerClass
    • SourceDebugExt
    • BootstrapMethods
    • Module
    • ModulePackages
    • ModuleMainClass
    • NestHost
    • NestMembers
    • Record
    • PermittedSubclasses
    • Synthetic
    • Deprecated
    • Signature
    • RuntimeVisibleAnnotations
    • RuntimeInvisibleAnnotations
    • RuntimeVisibleTypeAnnotations
    • RuntimeInvisibleTypeAnnotations, which is illegal."
    )]
    InvalidClassAttributes,
    #[error("Malformed class -> The fields in the Class file had attributes weren't from the following list
    • ConstantValue
    • Synthetic
    • Deprecated
    • Signature
    • RuntimeVisibleAnnotations
    • RuntimeInvisibleAnnotations
    • RuntimeVisibleTypeAnnotations
    • RuntimeVisibleTypeAnnotations"
    )]
    InvalidFieldInfoAttributes,
    #[error("TODO")]
    InvalidMethodInfoAttributes,
    #[error("TODO")]
    InvalidCodeAttributes,
    #[error("TODO")]
    InvalidRecordComponentInfoAttributes,
}
