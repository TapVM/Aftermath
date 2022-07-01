#![allow(dead_code)]

use super::errors;
pub use errors::Attributes as ErrorAttributes;
pub use errors::ParsingError;
use std::fmt::Debug;

type U1 = u8;
type U4 = u32;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct U2([u8; 2]);

impl Debug for U2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u2())
    }
}

impl U2 {
    pub fn to_u2(self) -> u16 {
        u16::from_be_bytes(self.0)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum CpNode<'class> {
    Class(Class),
    String(StringCp),
    MethodType(MethodType),
    Module(ModuleCp),
    Package(Package),
    Integer(Integer),
    Float(Float),
    Dynamic(Dynamic),
    NameAndType(NameAndType),
    InvokeDynamic(InvokeDynamic),
    FieldRef(Fieldref),
    MethodRef(Methodref),
    InterfaceMethodRef(InterfaceMethodref),
    Long(Long),
    Double(Double),
    MethodHandle(MethodHandle),
    Utf8(Utf8<'class>),
    None,
}

#[derive(Debug)]
pub enum Attributes<'class> {
    Value(Value),
    Code(AttrCode<'class>),
    StackMapTable(StackMapTable),
    Exceptions(Exceptions<'class>),
    InnerClass(InnerClass),
    EnclosingMethod(EnclosingMethod),
    Synthetic(Synthetic),
    Signature(Signature),
    SourceFile(SourceFile),
    SourceDebugExt(SourceDebugExt<'class>),
    LineNumberTable(LineNumberTable),
    LocalVariableTable(LocalVariableTable),
    LocalVariableTypeTable(LocalVariableTypeTable),
    Deprecated(Deprecated),
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotations),
    RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotations),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotations),
    RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotations),
    RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotations),
    AnnotationDefault(AnnotationDefault),
    BootstrapMethods(BootStrapMethods<'class>),
    MethodParameters(MethodParameters),
    Module(Module<'class>),
    ModulePackages(ModulePackages<'class>),
    ModuleMainClass(ModuleMainClass),
    NestHost(NestHost),
    NestMembers(NestMembers<'class>),
    Record(Record<'class>),
    PermittedSubclasses(PermittedSubclasses<'class>),
}

#[derive(Debug)]
pub enum TargetInfo {
    TypeParameterTarget(TypeParameterTarget),
    Supertype(Supertype),
    TypeParameterBound(TypeParameterBound),
    Empty,
    FormalParameter(FormalParameter),
    Throws(Throws),
    Localvar(Localvar),
    Catch(Catch),
    Offset(Offset),
    TypeArgument(TypeArgument),
}

#[derive(Debug)]
pub enum ElementValue {
    ConstValueIndex(U2),
    EnumConstValue(EnumConstValue),
    ClassInfoIndex(U2),
    AnnotationValue(Annotation),
    ArrayValue(ArrayValue),
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame(SameFrame),
    SameLocals1StackItemFrame(SameLocals1StackItemFrame),
    SameLocals1StackItemFrameExtended(SameLocals1StackItemFrameExtended),
    ChopFrame(ChopFrame),
    SameFrameExtended(SameFrameExtended),
    AppendFrame(AppendFrame),
    FullFrame(FullFrame),
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    TopVariableInfo(TopVariableInfo),
    IntegerVariableInfo(IntegerVariableInfo),
    FloatVariableInfo(FloatVariableInfo),
    DoubleVariableInfo(DoubleVariableInfo),
    LongVariableInfo(LongVariableInfo),
    NullVariableInfo(NullVariableInfo),
    UninitializedThisVariableInfo(UninitializedThisVariableInfo),
    ObjectVariableInfo(ObjectVariableInfo),
    UninitializedVariableInfo(UninitializedVariableInfo),
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SameFrame {
    pub frame_type: u8,
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrame {
    pub frame_type: u8,
    pub stack: VerificationTypeInfo,
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrameExtended {
    pub frame_type: u8,
    pub offset_delta: U2,
    pub stack: VerificationTypeInfo,
}

#[derive(Debug)]
pub struct ChopFrame {
    pub frame_type: u8,
    pub offset_delta: U2,
}

#[derive(Debug)]
pub struct SameFrameExtended {
    pub frame_type: u8,
    pub offset_delta: U2,
}

#[derive(Debug)]
pub struct AppendFrame {
    pub frame_type: u8,
    pub offset_delta: U2,
    pub locals: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub struct FullFrame {
    pub frame_type: u8,
    pub offset_delta: U2,
    pub locals: Vec<VerificationTypeInfo>,
    pub stack: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub struct TopVariableInfo {
    pub tag: u8,
}

#[derive(Debug)]
pub struct IntegerVariableInfo {
    pub tag: u8,
}

#[derive(Debug)]
pub struct FloatVariableInfo {
    pub tag: u8,
}

#[derive(Debug)]
pub struct DoubleVariableInfo {
    pub tag: u8,
}

#[derive(Debug)]
pub struct LongVariableInfo {
    pub tag: u8,
}

#[derive(Debug)]
pub struct NullVariableInfo {
    pub tag: u8,
}

#[derive(Debug)]
pub struct UninitializedThisVariableInfo {
    pub tag: u8,
}

#[derive(Debug)]
pub struct ObjectVariableInfo {
    pub tag: u8,
    pub cp_index: U2,
}

#[derive(Debug)]
pub struct UninitializedVariableInfo {
    pub tag: u8,
    pub offset: U2,
}

#[derive(Debug)]
pub struct PermittedSubclasses<'class> {
    pub classes: &'class [U2],
}

#[derive(Debug)]
pub struct RecordComponentInfo<'class> {
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes: Vec<Attributes<'class>>,
}

#[derive(Debug)]
pub struct Record<'class> {
    pub components: Vec<RecordComponentInfo<'class>>,
}

#[derive(Debug)]
pub struct ModulePackages<'class> {
    pub package_index: &'class [U2],
}

#[derive(Debug)]
pub struct ModuleMainClass {
    pub main_class_index: U2,
}

#[derive(Debug)]
pub struct NestHost {
    pub host_class_index: U2,
}

#[derive(Debug)]
pub struct NestMembers<'class> {
    pub classes: &'class [U2],
}

#[derive(Debug)]
pub struct ModuleRequires {
    pub requires_index: U2,
    pub requires_flags: U2,
    pub require_version_index: U2,
}

#[derive(Debug)]
pub struct ModuleExports<'class> {
    pub exports_index: U2,
    pub exports_flags: U2,
    pub exports_to_index: &'class [U2],
}

#[derive(Debug)]
pub struct ModuleOpens<'class> {
    pub opens_index: U2,
    pub opens_flags: U2,
    pub opens_to_index: &'class [U2],
}

#[derive(Debug)]
pub struct ModuleProvides<'class> {
    pub provides_index: U2,
    pub provides_with_index: &'class [U2],
}

#[derive(Debug)]
pub struct Module<'class> {
    pub module_name_index: U2,
    pub module_flags: U2,
    pub module_version_index: U2,
    pub requires: Vec<ModuleRequires>,
    pub exports: Vec<ModuleExports<'class>>,
    pub opens: Vec<ModuleOpens<'class>>,
    pub uses_index: &'class [U2],
    pub provides: Vec<ModuleProvides<'class>>,
}

#[derive(Debug)]
pub struct MethodParametersInner {
    pub name_index: U2,
    pub access_flags: U2,
}

#[derive(Debug)]
pub struct MethodParameters {
    pub parameters: Vec<MethodParametersInner>,
}

#[derive(Debug)]
pub struct AnnotationDefault {
    pub default_value: ElementValue,
}

#[derive(Debug)]
pub struct RuntimeInvisibleTypeAnnotations {
    pub annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct TypePathInner {
    pub type_path_kind: U1,
    pub type_argument_index: U1,
}

#[derive(Debug)]
pub struct TypePath {
    pub path: Vec<TypePathInner>,
}

#[derive(Debug)]
pub struct TypeAnnotationInner {
    pub element_name_index: U2,
    pub value: ElementValue,
}

#[derive(Debug)]
pub struct TypeAnnotation {
    pub target_info: TargetInfo,
    pub target_path: TypePath,
    pub type_index: U2,
    pub num_element_value_pairs: U2,
    pub element_value_pairs: Vec<TypeAnnotationInner>,
}

#[derive(Debug)]
pub struct RuntimeVisibleTypeAnnotations {
    pub type_annotation: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleAnnotations {
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct ParameterAnnotationsRuntimeParameterAnnotationsAttr {
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleParameterAnnotations {
    pub parameter_annotations: Vec<ParameterAnnotationsRuntimeParameterAnnotationsAttr>,
}

#[derive(Debug)]
pub struct RuntimeVisibleParameterAnnotations {
    pub parameter_annotations: Vec<ParameterAnnotationsRuntimeParameterAnnotationsAttr>,
}

#[derive(Debug)]
pub struct EnumConstValue {
    pub type_name_index: U2,
    pub const_name_index: U2,
}

#[derive(Debug)]
pub struct ArrayValue {
    pub element_value: Vec<ElementValue>,
}

#[derive(Debug)]
pub struct AnnotationInner {
    pub element_name_index: U2,
    pub value: ElementValue,
}

#[derive(Debug)]
pub struct Annotation {
    pub type_index: U2,
    pub element_value_pairs: Vec<AnnotationInner>,
}

#[derive(Debug)]
pub struct RuntimeVisibleAnnotations {
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct LineNumberTableAttrInner {
    pub start_pc: U2,
    pub line_number: U2,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableAttrInner {
    pub start_pc: U2,
    pub length: U2,
    pub name_index: U2,
    pub signature_index: U2,
    pub index: U2,
}

#[derive(Debug)]
pub struct LocalVariableTypeTable {
    pub local_variable_type_table: Vec<LocalVariableTypeTableAttrInner>,
}

#[derive(Debug)]
pub struct Deprecated;

#[derive(Debug)]
pub struct LocalVariableTableAttrInner {
    pub start_pc: U2,
    pub length: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub index: U2,
}

#[derive(Debug)]
pub struct LocalVariableTable {
    pub local_variable_table: Vec<LocalVariableTableAttrInner>,
}

#[derive(Debug)]
pub struct LineNumberTable {
    pub line_number_table: Vec<LineNumberTableAttrInner>,
}

#[derive(Debug)]
pub struct SourceDebugExt<'class> {
    pub debug_extension: &'class [U1],
}

#[derive(Debug)]
pub struct ExceptionTableAttrCode {
    pub start_pc: U2,
    pub end_pc: U2,
    pub handler_pc: U2,
    pub catch_type: U2,
}

#[derive(Debug)]
pub struct AttrCode<'class> {
    pub max_stack: U2,
    pub max_locals: U2,
    pub code: &'class [U1],
    pub exception_table: Vec<ExceptionTableAttrCode>,
    pub attributes: Vec<Attributes<'class>>,
}

#[derive(Debug)]
pub struct Synthetic;

#[derive(Debug)]
pub struct Signature {
    pub signature_index: U2,
}

#[derive(Debug)]
pub struct SourceFile {
    pub sourcefile_index: U2,
}

#[derive(Debug)]
pub struct EnclosingMethod {
    pub class_index: U2,
    pub method_index: U2,
}

#[derive(Debug)]
pub struct ClassesInnerClassAttr {
    pub inner_class_info_index: U2,
    pub outer_class_info_index: U2,
    pub inner_name_index: U2,
    pub inner_class_access_flags: U2,
}

#[derive(Debug)]
pub struct InnerClass {
    pub classes: Vec<ClassesInnerClassAttr>,
}

#[derive(Debug)]
pub struct StackMapTable {
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug)]
pub struct BootStrapMethodsInner<'class> {
    pub bootstrap_method_ref: U2,
    pub bootstrap_arguments: &'class [U2],
}

#[derive(Debug)]
pub struct BootStrapMethods<'class> {
    pub bootstrap_methods: Vec<BootStrapMethodsInner<'class>>,
}

#[derive(Debug)]
pub struct Value {
    pub value_index: U2,
}

#[derive(Debug)]
pub struct Exceptions<'class> {
    pub exception_index_table: &'class [U2],
}

#[derive(Debug)]
pub struct FieldInfo<'class> {
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes: Vec<Attributes<'class>>,
}

#[derive(Debug)]
pub struct MethodInfo<'class> {
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes: Vec<Attributes<'class>>,
}

#[derive(Debug)]
pub struct ClassFile<'class> {
    pub minor_v: U2,
    pub major_v: U2,
    pub cp: Vec<CpNode<'class>>,
    pub access_flags: U2,
    pub this_class: U2,
    pub super_class: U2,
    pub interfaces: &'class [U2],
    pub fields: Vec<FieldInfo<'class>>,
    pub methods: Vec<MethodInfo<'class>>,
    pub attributes: Vec<Attributes<'class>>,
}

#[derive(Debug)]
pub struct TypeParameterTarget {
    pub type_parameter_index: U1,
}

#[derive(Debug)]
pub struct Supertype {
    pub supertype_index: U2,
}

#[derive(Debug)]
pub struct TypeParameterBound {
    pub type_parameter_index: U1,
    pub bound_index: U1,
}

#[derive(Debug)]
pub struct FormalParameter {
    pub formal_parameter_index: U1,
}

#[derive(Debug)]
pub struct Throws {
    pub throws_type_index: U1,
}

#[derive(Debug)]
pub struct Localvar {
    pub table: Vec<LocalvarInner>,
}

#[derive(Debug)]
pub struct LocalvarInner {
    pub start_pc: U2,
    pub length: U2,
    pub index: U2,
}

#[derive(Debug)]
pub struct Catch {
    pub exception_table_index: U2,
}

#[derive(Debug)]
pub struct Offset {
    pub offset: U2,
}

#[derive(Debug)]
pub struct TypeArgument {
    pub offset: U2,
    pub type_argument_index: U1,
}

#[derive(Debug)]
pub struct Class {
    pub name_index: U2,
}

#[derive(Debug)]
pub struct Fieldref {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct Methodref {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct InterfaceMethodref {
    pub class_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct StringCp {
    pub string_index: U2,
}

#[derive(Debug)]
pub struct Integer {
    pub bytes: U4,
}

#[derive(Debug)]
pub struct Float {
    pub bytes: U4,
}

#[derive(Debug)]
pub struct Long {
    pub high_bytes: U4,
    pub low_bytes: U4,
}

#[derive(Debug)]
pub struct Double {
    pub high_bytes: U4,
    pub low_bytes: U4,
}

#[derive(Debug)]
pub struct NameAndType {
    pub name_index: U2,
    pub descriptor_index: U2,
}

#[derive(Debug)]
pub struct Utf8<'class> {
    pub bytes: &'class str,
}

#[derive(Debug)]
pub struct MethodHandle {
    pub reference_kind: u8,
    pub reference_index: U2,
}

#[derive(Debug)]
pub struct MethodType {
    pub descriptor_index: U2,
}

#[derive(Debug)]
pub struct Dynamic {
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct InvokeDynamic {
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ModuleCp {
    pub name_index: U2,
}

#[derive(Debug)]
pub struct Package {
    pub name_index: U2,
}
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Parser<'class> {
    pub bytes: &'class [U1],
    index: usize,
}

impl<'class> Utf8<'class> {
    pub fn verify_binary_class_or_interface_name(&self) -> Result<(), ParsingError<'class>> {
        for char in self.bytes.chars() {
            if char == '.' {
                return Err(ParsingError::BinaryNameContainsDot);
            }
        }

        Ok(())
    }
}

impl<'class> Parser<'class> {
    pub fn new(bytes: &'class [u8]) -> Self {
        Self { bytes, index: 0 }
    }

    fn u1(&mut self) -> U1 {
        let output = self.bytes[0];
        self.bytes = &self.bytes[1..];
        self.index += 1;
        output
    }

    fn u1_range(&mut self, length: U4) -> &'class [U1] {
        let output = &self.bytes[0..length as usize];
        self.bytes = &self.bytes[length as usize..];
        self.index += length as usize;

        output
    }

    fn u2(&mut self) -> U2 {
        U2([self.u1(), self.u1()])
    }

    fn u2_range(&mut self, length: U4) -> &'class [U2] {
        unsafe {
            core::slice::from_raw_parts(
                self.u1_range(length * 2).as_ptr().cast(),
                length.try_into().unwrap(), // This should be optimized away on 64 and 32 bit platforms since usize is >= U4
            )
        }
    }

    fn to_u2(&self, data: U2) -> u16 {
        u16::from_be_bytes(data.0)
    }

    fn u4(&mut self) -> U4 {
        U4::from_be_bytes(self.u1_range(4).try_into().unwrap())
    }

    fn element_value(&mut self) -> Result<ElementValue, ParsingError<'class>> {
        let tag = self.u1();

        match tag as char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 's' | 'Z' => {
                Ok(ElementValue::ConstValueIndex(self.u2()))
            }

            'e' => {
                let type_name_index = self.u2();
                let const_name_index = self.u2();

                Ok(ElementValue::EnumConstValue(EnumConstValue {
                    type_name_index,
                    const_name_index,
                }))
            }

            'c' => Ok(ElementValue::ClassInfoIndex(self.u2())),

            '@' => Ok(ElementValue::AnnotationValue(self.annotation()?)),

            '[' => {
                let length = self.u2();
                let mut values = Vec::with_capacity(length.to_u2().into());

                for _ in 0..length.to_u2() {
                    values.push(self.element_value()?);
                }

                Ok(ElementValue::ArrayValue(ArrayValue {
                    element_value: values,
                }))
            }

            _ => Err(ParsingError::InvalidElementValue(tag as char)),
        }
    }

    fn annotation(&mut self) -> Result<Annotation, ParsingError<'class>> {
        let type_index = self.u2();
        let num_element_value_pairs = self.u2();
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs.to_u2().into());

        for _ in 0..num_element_value_pairs.to_u2() {
            let element_name_index = self.u2();
            let value = self.element_value()?;

            element_value_pairs.push(AnnotationInner {
                element_name_index,
                value,
            })
        }

        Ok(Annotation {
            type_index,
            element_value_pairs,
        })
    }

    fn annotation_range(&mut self, length: u16) -> Result<Vec<Annotation>, ParsingError<'class>> {
        let mut annotations = Vec::with_capacity(length.into());

        for _ in 0..length {
            annotations.push(self.annotation()?);
        }

        Ok(annotations)
    }

    fn cp(&mut self, length: u16) -> Result<Vec<CpNode<'class>>, ParsingError<'class>> {
        let mut cp: Vec<CpNode<'class>> = Vec::with_capacity(length as usize - 1);

        while cp.len() + 1 < length as usize {
            let tag = self.u1();

            match tag {
                1 => {
                    let length = self.u2();
                    let bytes = self.u1_range(length.to_u2().into());

                    // TODO: mutf8 verification

                    cp.push(CpNode::Utf8(Utf8 {
                        bytes: std::str::from_utf8(bytes).unwrap(),
                    }))
                }

                7 => {
                    let name_index = self.u2();

                    cp.push(CpNode::Class(Class { name_index }))
                }

                9 => {
                    let class_index = self.u2();
                    let name_and_type_index = self.u2();

                    cp.push(CpNode::FieldRef(Fieldref {
                        class_index,
                        name_and_type_index,
                    }))
                }

                10 => {
                    let class_index = self.u2();
                    let name_and_type_index = self.u2();

                    cp.push(CpNode::MethodRef(Methodref {
                        class_index,
                        name_and_type_index,
                    }))
                }

                11 => {
                    let class_index = self.u2();
                    let name_and_type_index = self.u2();

                    cp.push(CpNode::InterfaceMethodRef(InterfaceMethodref {
                        class_index,
                        name_and_type_index,
                    }))
                }

                8 => {
                    let string_index = self.u2();

                    cp.push(CpNode::String(StringCp { string_index }));
                }

                3 => {
                    let bytes = self.u4();
                    cp.push(CpNode::Integer(Integer { bytes }));
                }

                4 => {
                    let bytes = self.u4();
                    cp.push(CpNode::Float(Float { bytes }));
                }

                5 => {
                    let high_bytes = self.u4();
                    let low_bytes = self.u4();

                    cp.push(CpNode::Long(Long {
                        high_bytes,
                        low_bytes,
                    }));

                    cp.push(CpNode::None);
                }

                6 => {
                    let high_bytes = self.u4();
                    let low_bytes = self.u4();

                    cp.push(CpNode::Double(Double {
                        high_bytes,
                        low_bytes,
                    }));

                    cp.push(CpNode::None);
                }

                12 => {
                    let name_index = self.u2();
                    let descriptor_index = self.u2();

                    cp.push(CpNode::NameAndType(NameAndType {
                        name_index,
                        descriptor_index,
                    }))
                }

                15 => {
                    let reference_kind = self.u1();
                    let reference_index = self.u2();

                    cp.push(CpNode::MethodHandle(MethodHandle {
                        reference_kind,
                        reference_index,
                    }))
                }

                16 => {
                    let descriptor_index = self.u2();

                    cp.push(CpNode::MethodType(MethodType { descriptor_index }))
                }

                17 => {
                    let bootstrap_method_attr_index = self.u2();
                    let name_and_type_index = self.u2();

                    cp.push(CpNode::Dynamic(Dynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    }));
                }

                18 => {
                    let bootstrap_method_attr_index = self.u2();
                    let name_and_type_index = self.u2();

                    cp.push(CpNode::InvokeDynamic(InvokeDynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    }));
                }

                19 => {
                    let name_index = self.u2();

                    cp.push(CpNode::Module(ModuleCp { name_index }))
                }

                20 => {
                    let name_index = self.u2();

                    cp.push(CpNode::Package(Package { name_index }))
                }

                _ => return Err(ParsingError::ConstantPoolTag(tag)),
            }
        }

        Ok(cp)
    }

    pub fn type_annotation(&mut self) -> Result<TypeAnnotation, ParsingError<'class>> {
        let target_type = self.u1();
        let target_info = match target_type {
            0x00 | 0x01 => {
                let type_parameter_index = self.u1();
                TargetInfo::TypeParameterTarget(TypeParameterTarget {
                    type_parameter_index,
                })
            }
            0x10 => {
                let supertype_index = self.u2();
                TargetInfo::Supertype(Supertype { supertype_index })
            }
            0x11 | 0x12 => {
                let type_parameter_index = self.u1();
                let bound_index = self.u1();

                TargetInfo::TypeParameterBound(TypeParameterBound {
                    type_parameter_index,
                    bound_index,
                })
            }
            0x13 | 0x14 | 0x15 => TargetInfo::Empty,
            0x16 => {
                let formal_parameter_index = self.u1();
                TargetInfo::FormalParameter(FormalParameter {
                    formal_parameter_index,
                })
            }
            0x17 => {
                let throws_type_index = self.u1();

                TargetInfo::Throws(Throws { throws_type_index })
            }
            0x40 | 0x41 => {
                let length = self.u2();
                let mut table = Vec::with_capacity(length.to_u2().into());

                for _ in 0..length.to_u2() {
                    let start_pc = self.u2();
                    let length = self.u2();
                    let index = self.u2();
                    table.push(LocalvarInner {
                        start_pc,
                        length,
                        index,
                    })
                }

                TargetInfo::Localvar(Localvar { table })
            }

            0x42 => {
                let exception_table_index = self.u2();

                TargetInfo::Catch(Catch {
                    exception_table_index,
                })
            }

            0x43 | 0x44 | 0x45 | 0x46 => {
                let offset = self.u2();

                TargetInfo::Offset(Offset { offset })
            }

            0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
                let offset = self.u2();
                let type_argument_index = self.u1();

                TargetInfo::TypeArgument(TypeArgument {
                    offset,
                    type_argument_index,
                })
            }
            _ => return Err(ParsingError::InvalidTargetType(target_type)),
        };

        let type_path_length = self.u1();
        let mut path = Vec::with_capacity(type_path_length.into());

        for _ in 0..type_path_length {
            let type_path_kind = self.u1();
            let type_argument_index = self.u1();
            path.push(TypePathInner {
                type_path_kind,
                type_argument_index,
            })
        }

        let target_path = TypePath { path };

        let type_index = self.u2();
        let num_element_value_pairs = self.u2();
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs.to_u2().into());

        for _ in 0..num_element_value_pairs.to_u2() {
            let element_name_index = self.u2();
            let value = self.element_value()?;

            element_value_pairs.push(TypeAnnotationInner {
                element_name_index,
                value,
            })
        }

        Ok(TypeAnnotation {
            target_info,
            target_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }

    fn type_annotation_range(
        &mut self,
        length: u16,
    ) -> Result<Vec<TypeAnnotation>, ParsingError<'class>> {
        let mut annotations = Vec::with_capacity(length.into());

        for _ in 0..length {
            annotations.push(self.type_annotation()?);
        }

        Ok(annotations)
    }

    fn stackmapframe(&mut self) -> Result<StackMapFrame, ParsingError<'class>> {
        let frame_type = self.u1();

        match frame_type {
            0..=63 => Ok(StackMapFrame::SameFrame(SameFrame { frame_type })),
            64..=127 => Ok(StackMapFrame::SameLocals1StackItemFrame(
                SameLocals1StackItemFrame {
                    frame_type,
                    stack: self.verification_type_info()?,
                },
            )),
            247 => {
                let offset_delta = self.u2();
                let stack = self.verification_type_info()?;

                Ok(StackMapFrame::SameLocals1StackItemFrameExtended(
                    SameLocals1StackItemFrameExtended {
                        frame_type,
                        offset_delta,
                        stack,
                    },
                ))
            }
            248..=250 => Ok(StackMapFrame::ChopFrame(ChopFrame {
                frame_type,
                offset_delta: self.u2(),
            })),
            251 => Ok(StackMapFrame::SameFrameExtended(SameFrameExtended {
                frame_type,
                offset_delta: self.u2(),
            })),
            252..=254 => {
                let offset_delta = self.u2();
                let length = frame_type - 251;
                let mut locals = Vec::with_capacity(length as usize);

                for _ in 0..length {
                    locals.push(self.verification_type_info()?);
                }

                Ok(StackMapFrame::AppendFrame(AppendFrame {
                    frame_type,
                    offset_delta,
                    locals,
                }))
            }
            255 => {
                let offset_delta = self.u2();

                let length = self.u2();
                let mut locals = Vec::with_capacity(length.to_u2() as usize);

                for _ in 0..length.to_u2() {
                    locals.push(self.verification_type_info()?);
                }

                let length = self.u2();
                let mut stack = Vec::with_capacity(length.to_u2() as usize);

                for _ in 0..length.to_u2() {
                    stack.push(self.verification_type_info()?);
                }

                Ok(StackMapFrame::FullFrame(FullFrame {
                    frame_type,
                    offset_delta,
                    locals,
                    stack,
                }))
            }
            _ => Err(ParsingError::InvalidFrameType(frame_type)),
        }
    }

    fn verification_type_info(&mut self) -> Result<VerificationTypeInfo, ParsingError<'class>> {
        let tag = self.u1();

        Ok(match tag {
            0 => VerificationTypeInfo::TopVariableInfo(TopVariableInfo { tag }),
            1 => VerificationTypeInfo::IntegerVariableInfo(IntegerVariableInfo { tag }),
            2 => VerificationTypeInfo::FloatVariableInfo(FloatVariableInfo { tag }),
            3 => VerificationTypeInfo::DoubleVariableInfo(DoubleVariableInfo { tag }),
            4 => VerificationTypeInfo::LongVariableInfo(LongVariableInfo { tag }),
            5 => VerificationTypeInfo::NullVariableInfo(NullVariableInfo { tag }),
            6 => {
                VerificationTypeInfo::UninitializedThisVariableInfo(UninitializedThisVariableInfo {
                    tag,
                })
            }
            7 => VerificationTypeInfo::ObjectVariableInfo(ObjectVariableInfo {
                tag,
                cp_index: self.u2(),
            }),
            8 => VerificationTypeInfo::UninitializedVariableInfo(UninitializedVariableInfo {
                tag,
                offset: self.u2(),
            }),
            _ => Err(ParsingError::InvalidTagVerificationTypeInfo(tag))?,
        })
    }

    fn attributes(
        &mut self,
        length: u16,
        cp: &Vec<CpNode>,
    ) -> Result<Vec<Attributes<'class>>, ParsingError<'class>> {
        let mut attributes = Vec::with_capacity(length as usize);

        for _ in 0..length as usize {
            let attribute_name_index = self.u2();
            let attribute_length = self.u4();
            let tag = &cp[attribute_name_index.to_u2() as usize - 1];

            if let CpNode::Utf8(tag) = tag {
                match tag.bytes {
                    "ConstantValue" => {
                        let value_index = self.u2();

                        attributes.push(Attributes::Value(Value { value_index }))
                    }

                    "SourceFile" => attributes.push(Attributes::SourceFile(SourceFile {
                        sourcefile_index: self.u2(),
                    })),

                    "Module" => {
                        let module_name_index = self.u2();
                        let module_flags = self.u2();
                        let module_version_index = self.u2();

                        let requires_count = self.u2();
                        let mut requires = Vec::with_capacity(requires_count.to_u2().into());

                        for _ in 0..requires_count.to_u2() {
                            let requires_index = self.u2();
                            let requires_flags = self.u2();
                            let require_version_index = self.u2();

                            requires.push(ModuleRequires {
                                requires_index,
                                requires_flags,
                                require_version_index,
                            });
                        }

                        let exports_count = self.u2();
                        let mut exports = Vec::with_capacity(exports_count.to_u2().into());
                        for _ in 0..exports_count.to_u2() {
                            let exports_index = self.u2();
                            let exports_flags = self.u2();
                            let exports_to_count = self.u2();
                            let exports_to_index = self.u2_range(exports_to_count.to_u2().into());

                            exports.push(ModuleExports {
                                exports_index,
                                exports_flags,
                                exports_to_index,
                            });
                        }

                        let opens_count = self.u2();
                        let mut opens = Vec::with_capacity(opens_count.to_u2().into());

                        for _ in 0..opens_count.to_u2() {
                            let opens_index = self.u2();
                            let opens_flags = self.u2();
                            let opens_to_count = self.u2();
                            let opens_to_index = self.u2_range(opens_to_count.to_u2().into());

                            opens.push(ModuleOpens {
                                opens_index,
                                opens_flags,
                                opens_to_index,
                            })
                        }

                        let uses_count = self.u2();
                        let uses_index = self.u2_range(uses_count.to_u2().into());

                        let provides_count = self.u2();
                        let mut provides = Vec::with_capacity(provides_count.to_u2().into());

                        for _ in 0..provides_count.to_u2() {
                            let provides_index = self.u2();
                            let provides_with_count = self.u2();
                            let provides_with_index =
                                self.u2_range(provides_with_count.to_u2().into());

                            provides.push(ModuleProvides {
                                provides_index,
                                provides_with_index,
                            });
                        }

                        attributes.push(Attributes::Module(Module {
                            module_name_index,
                            module_flags,
                            module_version_index,
                            requires,
                            exports,
                            opens,
                            uses_index,
                            provides,
                        }))
                    }

                    "Code" => {
                        let max_stack = self.u2();
                        let max_locals = self.u2();
                        let code_length = self.u4();
                        let code = self.u1_range(code_length);
                        let exception_table_length = self.u2();
                        let mut exception_table =
                            Vec::with_capacity(exception_table_length.to_u2().into());

                        for _ in 0..exception_table_length.to_u2() {
                            let start_pc = self.u2();
                            let end_pc = self.u2();
                            let handler_pc = self.u2();
                            let catch_type = self.u2();

                            exception_table.push(ExceptionTableAttrCode {
                                start_pc,
                                end_pc,
                                handler_pc,
                                catch_type,
                            });
                        }

                        let attributes_count = self.u2();
                        let local_attributes = self.attributes(attributes_count.to_u2(), cp)?;

                        attributes.push(Attributes::Code(AttrCode {
                            max_stack,
                            max_locals,
                            code,
                            exception_table,
                            attributes: local_attributes,
                        }))
                    }

                    "LineNumberTable" => {
                        let line_number_table_length = self.u2();
                        let mut line_number_table =
                            Vec::with_capacity(line_number_table_length.to_u2().into());

                        for _ in 0..line_number_table_length.to_u2() {
                            let start_pc = self.u2();
                            let line_number = self.u2();

                            line_number_table.push(LineNumberTableAttrInner {
                                start_pc,
                                line_number,
                            })
                        }

                        attributes.push(Attributes::LineNumberTable(LineNumberTable {
                            line_number_table,
                        }))
                    }

                    "StackMapTable" => {
                        let number_of_entries = self.u2();
                        let mut entries = Vec::with_capacity(number_of_entries.to_u2().into());

                        for _ in 0..number_of_entries.to_u2() {
                            entries.push(self.stackmapframe()?);
                        }

                        attributes.push(Attributes::StackMapTable(StackMapTable { entries }))
                    }

                    "Exceptions" => {
                        let number_of_exceptions = self.u2();
                        let exception_index_table =
                            self.u2_range(number_of_exceptions.to_u2().into());

                        attributes.push(Attributes::Exceptions(Exceptions {
                            exception_index_table,
                        }))
                    }

                    "InnerClasses" => {
                        let number_of_classes = self.u2();
                        let mut classes = Vec::with_capacity(number_of_classes.to_u2().into());

                        for _ in 0..number_of_classes.to_u2() {
                            let inner_class_info_index = self.u2();
                            let outer_class_info_index = self.u2();
                            let inner_name_index = self.u2();
                            let inner_class_access_flags = self.u2();

                            classes.push(ClassesInnerClassAttr {
                                inner_class_info_index,
                                outer_class_info_index,
                                inner_name_index,
                                inner_class_access_flags,
                            })
                        }

                        attributes.push(Attributes::InnerClass(InnerClass { classes }))
                    }

                    "EnclosingMethod" => {
                        let class_index = self.u2();
                        let method_index = self.u2();

                        attributes.push(Attributes::EnclosingMethod(EnclosingMethod {
                            class_index,
                            method_index,
                        }))
                    }

                    "Synthetic" => attributes.push(Attributes::Synthetic(Synthetic)),

                    "Signature" => attributes.push(Attributes::Signature(Signature {
                        signature_index: self.u2(),
                    })),

                    "SourceDebugExtension" => {
                        attributes.push(Attributes::SourceDebugExt(SourceDebugExt {
                            debug_extension: self.u1_range(attribute_length),
                        }))
                    }

                    "LocalVariableTable" => {
                        let length = self.u2();
                        let mut local_variable_table = Vec::with_capacity(length.to_u2().into());

                        for _ in 0..length.to_u2() {
                            let start_pc = self.u2();
                            let length = self.u2();
                            let name_index = self.u2();
                            let descriptor_index = self.u2();
                            let index = self.u2();

                            local_variable_table.push(LocalVariableTableAttrInner {
                                start_pc,
                                length,
                                name_index,
                                descriptor_index,
                                index,
                            })
                        }

                        attributes.push(Attributes::LocalVariableTable(LocalVariableTable {
                            local_variable_table,
                        }))
                    }

                    "LocalVariableTypeTable" => {
                        let length = self.u2();
                        let mut local_variable_type_table =
                            Vec::with_capacity(length.to_u2().into());

                        for _ in 0..length.to_u2() {
                            let start_pc = self.u2();
                            let length = self.u2();
                            let name_index = self.u2();
                            let signature_index = self.u2();
                            let index = self.u2();

                            local_variable_type_table.push(LocalVariableTypeTableAttrInner {
                                start_pc,
                                length,
                                name_index,
                                signature_index,
                                index,
                            })
                        }

                        attributes.push(Attributes::LocalVariableTypeTable(
                            LocalVariableTypeTable {
                                local_variable_type_table,
                            },
                        ))
                    }

                    "Deprecated" => attributes.push(Attributes::Deprecated(Deprecated)),

                    "RuntimeVisibleAnnotations" => {
                        let length = self.u2();
                        let annotations = self.annotation_range(length.to_u2())?;

                        attributes.push(Attributes::RuntimeVisibleAnnotations(
                            RuntimeVisibleAnnotations { annotations },
                        ))
                    }

                    "RuntimeInvisibleAnnotations" => {
                        let length = self.u2();
                        let annotations = self.annotation_range(length.to_u2())?;

                        attributes.push(Attributes::RuntimeInvisibleAnnotations(
                            RuntimeInvisibleAnnotations { annotations },
                        ))
                    }

                    "RuntimeVisibleParameterAnnotations" => {
                        let length = self.u1();
                        let mut parameter_annotations = Vec::with_capacity(length.into());

                        for _ in 0..length {
                            let length = self.u2();
                            let annotations = self.annotation_range(length.to_u2())?;
                            parameter_annotations.push(
                                ParameterAnnotationsRuntimeParameterAnnotationsAttr { annotations },
                            )
                        }

                        attributes.push(Attributes::RuntimeVisibleParameterAnnotations(
                            RuntimeVisibleParameterAnnotations {
                                parameter_annotations,
                            },
                        ))
                    }

                    "RuntimeInvisibleParameterAnnotations" => {
                        let length = self.u1();
                        let mut parameter_annotations = Vec::with_capacity(length.into());

                        for _ in 0..length {
                            let length = self.u2();
                            let annotations = self.annotation_range(length.to_u2())?;
                            parameter_annotations.push(
                                ParameterAnnotationsRuntimeParameterAnnotationsAttr { annotations },
                            )
                        }

                        attributes.push(Attributes::RuntimeInvisibleParameterAnnotations(
                            RuntimeInvisibleParameterAnnotations {
                                parameter_annotations,
                            },
                        ))
                    }

                    "RuntimeVisibleTypeAnnotations" => {
                        let length = self.u2();
                        let annotations = self.type_annotation_range(length.to_u2())?;

                        attributes.push(Attributes::RuntimeVisibleTypeAnnotations(
                            RuntimeVisibleTypeAnnotations {
                                type_annotation: annotations,
                            },
                        ))
                    }

                    "RuntimeInvisibleTypeAnnotations" => {
                        let length = self.u2();
                        let annotations = self.type_annotation_range(length.to_u2())?;

                        attributes.push(Attributes::RuntimeInvisibleTypeAnnotations(
                            RuntimeInvisibleTypeAnnotations { annotations },
                        ))
                    }

                    "AnnotationDefault" => {
                        let default_value = self.element_value()?;

                        attributes.push(Attributes::AnnotationDefault(AnnotationDefault {
                            default_value,
                        }))
                    }

                    "BootstrapMethods" => {
                        let length = self.u2();
                        let mut bootstrap_methods = Vec::with_capacity(length.to_u2().into());

                        for _ in 0..length.to_u2() {
                            let bootstrap_method_ref = self.u2();
                            let num_bootstrap_arguments = self.u2();
                            let bootstrap_arguments =
                                self.u2_range(num_bootstrap_arguments.to_u2().into());

                            bootstrap_methods.push(BootStrapMethodsInner {
                                bootstrap_method_ref,
                                bootstrap_arguments,
                            })
                        }

                        attributes.push(Attributes::BootstrapMethods(BootStrapMethods {
                            bootstrap_methods,
                        }))
                    }

                    "MethodParameters" => {
                        let length = self.u1();
                        let mut parameters = Vec::with_capacity(length.into());

                        for _ in 0..length {
                            let name_index = self.u2();
                            let access_flags = self.u2();

                            parameters.push(MethodParametersInner {
                                name_index,
                                access_flags,
                            })
                        }

                        attributes.push(Attributes::MethodParameters(MethodParameters {
                            parameters,
                        }))
                    }

                    "ModulePackages" => {
                        let package_count = self.u2();
                        let package_index = self.u2_range(package_count.to_u2().into());

                        attributes
                            .push(Attributes::ModulePackages(ModulePackages { package_index }))
                    }

                    "ModuleMainClass" => {
                        let main_class_index = self.u2();

                        attributes.push(Attributes::ModuleMainClass(ModuleMainClass {
                            main_class_index,
                        }))
                    }

                    "NestHost" => {
                        let host_class_index = self.u2();

                        attributes.push(Attributes::NestHost(NestHost { host_class_index }))
                    }

                    "NestMembers" => {
                        let number_of_classes = self.u2();
                        let classes = self.u2_range(number_of_classes.to_u2().into());
                        attributes.push(Attributes::NestMembers(NestMembers { classes }))
                    }

                    "Record" => {
                        let count = self.u2();
                        let mut components = Vec::with_capacity(count.to_u2().into());

                        for _ in 0..count.to_u2() {
                            let name_index = self.u2();
                            let descriptor_index = self.u2();
                            let attributes_count = self.u2();
                            let attributes = self.attributes(attributes_count.to_u2(), cp)?;

                            components.push(RecordComponentInfo {
                                name_index,
                                descriptor_index,
                                attributes,
                            })
                        }

                        attributes.push(Attributes::Record(Record { components }))
                    }

                    "PermittedSubclasses" => {
                        let length = self.u2();
                        let classes = self.u2_range(length.to_u2().into());

                        attributes.push(Attributes::PermittedSubclasses(PermittedSubclasses {
                            classes,
                        }))
                    }

                    _ => return Err(ParsingError::InvalidAttribute(tag.bytes.to_string())),
                }
            } else {
                return Err(ParsingError::AttributeNotUtf8);
            }
        }

        Ok(attributes)
    }

    fn methods(
        &mut self,
        length: u16,
        cp: &Vec<CpNode<'class>>,
    ) -> Result<Vec<MethodInfo<'class>>, ParsingError<'class>> {
        let mut methods = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();
            let attributes = self.attributes(attributes_count.to_u2(), cp)?;
            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            })
        }

        Ok(methods)
    }

    fn fields(
        &mut self,
        length: u16,
        cp: &Vec<CpNode<'class>>,
    ) -> Result<Vec<FieldInfo<'class>>, ParsingError<'class>> {
        let mut fields = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();
            let attributes = self.attributes(attributes_count.to_u2(), cp)?;

            fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            })
        }

        Ok(fields)
    }

    pub fn parse(&mut self) -> Result<ClassFile<'class>, ParsingError<'class>> {
        let magic = self.u4();

        if magic != 0xCAFEBABE {
            return Err(ParsingError::Magic);
        }

        let minor_v = self.u2();
        let major_v = self.u2();

        let cp_count = self.u2();
        let cp = self.cp(cp_count.to_u2())?;

        let access_flags = self.u2();
        let this_class = self.u2();
        let super_class = self.u2();

        let interfaces_count = self.u2();
        let interfaces = self.u2_range(interfaces_count.to_u2() as u32);

        let fields_count = self.u2();
        let fields = self.fields(fields_count.to_u2(), &cp)?;

        let methods_count = self.u2();
        let methods = self.methods(methods_count.to_u2(), &cp)?;

        let attributes_count = self.u2();
        let attributes = self.attributes(attributes_count.to_u2(), &cp)?;

        Ok(ClassFile {
            minor_v,
            major_v,
            cp,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}
