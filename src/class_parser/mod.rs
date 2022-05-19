#![allow(dead_code)]

mod errors;

use std::fmt::Debug;

pub use errors::ParsingError;

type U1 = u8;
type U4 = u32;
type Result<T, E = ParsingError> = core::result::Result<T, E>;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct U2([u8; 2]);

impl Debug for U2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u16::from_be_bytes(self.0))
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
    frame_type: u8,
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrame {
    frame_type: u8,
    stack: VerificationTypeInfo,
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrameExtended {
    frame_type: u8,
    offset_delta: U2,
    stack: VerificationTypeInfo,
}

#[derive(Debug)]
pub struct ChopFrame {
    frame_type: u8,
    offset_delta: U2,
}

#[derive(Debug)]
pub struct SameFrameExtended {
    frame_type: u8,
    offset_delta: U2,
}

#[derive(Debug)]
pub struct AppendFrame {
    frame_type: u8,
    offset_delta: U2,
    locals: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub struct FullFrame {
    frame_type: u8,
    offset_delta: U2,
    locals: Vec<VerificationTypeInfo>,
    stack: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
pub struct TopVariableInfo {
    tag: u8,
}

#[derive(Debug)]
pub struct IntegerVariableInfo {
    tag: u8,
}

#[derive(Debug)]
pub struct FloatVariableInfo {
    tag: u8,
}

#[derive(Debug)]
pub struct DoubleVariableInfo {
    tag: u8,
}

#[derive(Debug)]
pub struct LongVariableInfo {
    tag: u8,
}

#[derive(Debug)]
pub struct NullVariableInfo {
    tag: u8,
}

#[derive(Debug)]
pub struct UninitializedThisVariableInfo {
    tag: u8,
}

#[derive(Debug)]
pub struct ObjectVariableInfo {
    tag: u8,
    cp_index: U2,
}

#[derive(Debug)]
pub struct UninitializedVariableInfo {
    tag: u8,
    offset: U2,
}

#[derive(Debug)]
pub struct PermittedSubclasses<'class> {
    classes: &'class [U2],
}

#[derive(Debug)]
pub struct RecordComponentInfo<'class> {
    name_index: U2,
    descriptor_index: U2,
    attributes: Vec<Attributes<'class>>,
}

#[derive(Debug)]
pub struct Record<'class> {
    components: Vec<RecordComponentInfo<'class>>,
}

#[derive(Debug)]
pub struct ModulePackages<'class> {
    package_index: &'class [U2],
}

#[derive(Debug)]
pub struct ModuleMainClass {
    main_class_index: U2,
}

#[derive(Debug)]
pub struct NestHost {
    host_class_index: U2,
}

#[derive(Debug)]
pub struct NestMembers<'class> {
    classes: &'class [U2],
}

#[derive(Debug)]
pub struct ModuleRequires {
    requires_index: U2,
    requires_flags: U2,
    require_version_index: U2,
}

#[derive(Debug)]
pub struct ModuleExports<'class> {
    exports_index: U2,
    exports_flags: U2,
    exports_to_index: &'class [U2],
}

#[derive(Debug)]
pub struct ModuleOpens<'class> {
    opens_index: U2,
    opens_flags: U2,
    opens_to_index: &'class [U2],
}

#[derive(Debug)]
pub struct ModuleProvides<'class> {
    provides_index: U2,
    provides_with_index: &'class [U2],
}

#[derive(Debug)]
pub struct Module<'class> {
    module_name_index: U2,
    module_flags: U2,
    module_version_index: U2,
    requires: Vec<ModuleRequires>,
    exports: Vec<ModuleExports<'class>>,
    opens: Vec<ModuleOpens<'class>>,
    uses_index: &'class [U2],
    provides: Vec<ModuleProvides<'class>>,
}

#[derive(Debug)]
pub struct MethodParametersInner {
    name_index: U2,
    access_flags: U2,
}

#[derive(Debug)]
pub struct MethodParameters {
    parameters: Vec<MethodParametersInner>,
}

#[derive(Debug)]
pub struct AnnotationDefault {
    default_value: ElementValue,
}

#[derive(Debug)]
pub struct RuntimeInvisibleTypeAnnotations {
    annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct TypePathInner {
    type_path_kind: U1,
    type_argument_index: U1,
}

#[derive(Debug)]
pub struct TypePath {
    path: Vec<TypePathInner>,
}

#[derive(Debug)]
pub struct TypeAnnotationInner {
    element_name_index: U2,
    value: ElementValue,
}

#[derive(Debug)]
pub struct TypeAnnotation {
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: U2,
    num_element_value_pairs: U2,
    element_value_pairs: Vec<TypeAnnotationInner>,
}

#[derive(Debug)]
pub struct RuntimeVisibleTypeAnnotations {
    type_annotation: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleAnnotations {
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct ParameterAnnotationsRuntimeParameterAnnotationsAttr {
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleParameterAnnotations {
    parameter_annotations: Vec<ParameterAnnotationsRuntimeParameterAnnotationsAttr>,
}

#[derive(Debug)]
pub struct RuntimeVisibleParameterAnnotations {
    parameter_annotations: Vec<ParameterAnnotationsRuntimeParameterAnnotationsAttr>,
}

#[derive(Debug)]
pub struct EnumConstValue {
    type_name_index: U2,
    const_name_index: U2,
}

#[derive(Debug)]
pub struct ArrayValue {
    element_value: Vec<ElementValue>,
}

#[derive(Debug)]
pub struct AnnotationInner {
    element_name_index: U2,
    value: ElementValue,
}

#[derive(Debug)]
pub struct Annotation {
    type_index: U2,
    element_value_pairs: Vec<AnnotationInner>,
}

#[derive(Debug)]
pub struct RuntimeVisibleAnnotations {
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct LineNumberTableAttrInner {
    start_pc: U2,
    line_number: U2,
}

#[derive(Debug)]
pub struct LocalVariableTypeTableAttrInner {
    start_pc: U2,
    length: U2,
    name_index: U2,
    signature_index: U2,
    index: U2,
}

#[derive(Debug)]
pub struct LocalVariableTypeTable {
    local_variable_type_table: Vec<LocalVariableTypeTableAttrInner>,
}

#[derive(Debug)]
pub struct Deprecated;

#[derive(Debug)]
pub struct LocalVariableTableAttrInner {
    start_pc: U2,
    length: U2,
    name_index: U2,
    descriptor_index: U2,
    index: U2,
}

#[derive(Debug)]
pub struct LocalVariableTable {
    local_variable_table: Vec<LocalVariableTableAttrInner>,
}

#[derive(Debug)]
pub struct LineNumberTable {
    line_number_table: Vec<LineNumberTableAttrInner>,
}

#[derive(Debug)]
pub struct SourceDebugExt<'class> {
    debug_extension: &'class [U1],
}

#[derive(Debug)]
pub struct ExceptionTableAttrCode {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2,
}

#[derive(Debug)]
pub struct AttrCode<'class> {
    max_stack: U2,
    max_locals: U2,
    code: &'class [U1],
    exception_table: Vec<ExceptionTableAttrCode>,
    attributes: Vec<Attributes<'class>>,
}

#[derive(Debug)]
pub struct Synthetic;

#[derive(Debug)]
pub struct Signature {
    signature_index: U2,
}

#[derive(Debug)]
pub struct SourceFile {
    sourcefile_index: U2,
}

#[derive(Debug)]
pub struct EnclosingMethod {
    class_index: U2,
    method_index: U2,
}

#[derive(Debug)]
pub struct ClassesInnerClassAttr {
    inner_class_info_index: U2,
    outer_class_info_index: U2,
    inner_name_index: U2,
    inner_class_access_flags: U2,
}

#[derive(Debug)]
pub struct InnerClass {
    classes: Vec<ClassesInnerClassAttr>,
}

#[derive(Debug)]
pub struct StackMapTable {
    entries: Vec<StackMapFrame>,
}

#[derive(Debug)]
pub struct BootStrapMethodsInner<'class> {
    bootstrap_method_ref: U2,
    bootstrap_arguments: &'class [U2],
}

#[derive(Debug)]
pub struct BootStrapMethods<'class> {
    bootstrap_methods: Vec<BootStrapMethodsInner<'class>>,
}

#[derive(Debug)]
pub struct Value {
    value_index: U2,
}

#[derive(Debug)]
pub struct Exceptions<'class> {
    exception_index_table: &'class [U2],
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
    type_parameter_index: U1,
}

#[derive(Debug)]
pub struct Supertype {
    supertype_index: U2,
}

#[derive(Debug)]
pub struct TypeParameterBound {
    type_parameter_index: U1,
    bound_index: U1,
}

#[derive(Debug)]
pub struct FormalParameter {
    formal_parameter_index: U1,
}

#[derive(Debug)]
pub struct Throws {
    throws_type_index: U1,
}

#[derive(Debug)]
pub struct Localvar {
    table: Vec<LocalvarInner>,
}

#[derive(Debug)]
pub struct LocalvarInner {
    start_pc: U2,
    length: U2,
    index: U2,
}

#[derive(Debug)]
pub struct Catch {
    exception_table_index: U2,
}

#[derive(Debug)]
pub struct Offset {
    offset: U2,
}

#[derive(Debug)]
pub struct TypeArgument {
    offset: U2,
    type_argument_index: U1,
}

#[derive(Debug)]
pub struct Class {
    name_index: U2,
}

#[derive(Debug)]
pub struct Fieldref {
    class_index: U2,
    name_and_type_index: U2,
}

#[derive(Debug)]
pub struct Methodref {
    class_index: U2,
    name_and_type_index: U2,
}

#[derive(Debug)]
pub struct InterfaceMethodref {
    class_index: U2,
    name_and_type_index: U2,
}

#[derive(Debug)]
pub struct StringCp {
    string_index: U2,
}

#[derive(Debug)]
pub struct Integer {
    bytes: U4,
}

#[derive(Debug)]
pub struct Float {
    bytes: U4,
}

#[derive(Debug)]
pub struct Long {
    high_bytes: U4,
    low_bytes: U4,
}

#[derive(Debug)]
pub struct Double {
    high_bytes: U4,
    low_bytes: U4,
}

#[derive(Debug)]
pub struct NameAndType {
    name_index: U2,
    descriptor_index: U2,
}

#[derive(Debug)]
pub struct Utf8<'class> {
    bytes: &'class str,
}

#[derive(Debug)]
pub struct MethodHandle {
    reference_kind: u8,
    reference_index: U2,
}

#[derive(Debug)]
pub struct MethodType {
    descriptor_index: U2,
}

#[derive(Debug)]
pub struct Dynamic {
    bootstrap_method_attr_index: U2,
    name_and_type_index: U2,
}

#[derive(Debug)]
pub struct InvokeDynamic {
    bootstrap_method_attr_index: U2,
    name_and_type_index: U2,
}

#[derive(Debug)]
pub struct ModuleCp {
    name_index: U2,
}

#[derive(Debug)]
pub struct Package {
    name_index: U2,
}
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Parser<'class> {
    pub bytes: &'class [U1],
    index: usize,
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
                length.try_into().unwrap(),
            )
        }
    }

    fn to_u2(&self, data: U2) -> u16 {
        u16::from_be_bytes(data.0)
    }

    fn u4(&mut self) -> U4 {
        U4::from_be_bytes(self.u1_range(4).try_into().unwrap())
    }

    fn element_value(&mut self) -> ElementValue {
        let tag = self.u1();

        match tag as char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 's' => {
                ElementValue::ConstValueIndex(self.u2())
            }

            'e' => {
                let type_name_index = self.u2();
                let const_name_index = self.u2();

                ElementValue::EnumConstValue(EnumConstValue {
                    type_name_index,
                    const_name_index,
                })
            }

            'c' => ElementValue::ClassInfoIndex(self.u2()),

            '@' => ElementValue::AnnotationValue(self.annotation()),

            '[' => {
                let length = self.u2();
                let mut values = Vec::with_capacity(self.to_u2(length).into());

                for _ in 0..self.to_u2(length) {
                    println!("[");
                    values.push(self.element_value());
                }

                ElementValue::ArrayValue(ArrayValue {
                    element_value: values,
                })
            }

            _ => unreachable!("{}", tag as char),
        }
    }

    fn annotation(&mut self) -> Annotation {
        let type_index = self.u2();
        let num_element_value_pairs = self.u2();
        let mut element_value_pairs =
            Vec::with_capacity(self.to_u2(num_element_value_pairs).into());

        for _ in 0..self.to_u2(num_element_value_pairs) {
            let element_name_index = self.u2();
            println!("RIP");
            let value = self.element_value();

            element_value_pairs.push(AnnotationInner {
                element_name_index,
                value,
            })
        }

        Annotation {
            type_index,
            element_value_pairs,
        }
    }

    fn annotation_range(&mut self, length: u16) -> Vec<Annotation> {
        let mut annotations = Vec::with_capacity(length.into());

        for _ in 0..length {
            annotations.push(self.annotation());
        }

        annotations
    }

    fn cp(&mut self, length: u16) -> Vec<CpNode<'class>> {
        let mut cp: Vec<CpNode<'class>> = Vec::with_capacity(length as usize - 1);

        while cp.len() + 1 < length as usize {
            let tag = self.u1();

            match tag {
                1 => {
                    let length = self.u2();
                    let bytes = self.u1_range(self.to_u2(length).into());
                    cp.push(CpNode::Utf8(Utf8 {
                        bytes: std::str::from_utf8(&bytes).unwrap(),
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

                _ => unreachable!("Constant Pool -> {}", tag),
            }
        }

        cp
    }

    pub fn type_annotation(&mut self) -> TypeAnnotation {
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
                let mut table = Vec::with_capacity(self.to_u2(length).into());

                for _ in 0..self.to_u2(length) {
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
            _ => unreachable!(),
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
        let mut element_value_pairs =
            Vec::with_capacity(self.to_u2(num_element_value_pairs).into());

        for _ in 0..self.to_u2(num_element_value_pairs) {
            let element_name_index = self.u2();
            dbg!();
            let value = self.element_value();

            element_value_pairs.push(TypeAnnotationInner {
                element_name_index,
                value,
            })
        }

        TypeAnnotation {
            target_info,
            target_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        }
    }

    fn type_annotation_range(&mut self, length: u16) -> Vec<TypeAnnotation> {
        let mut annotations = Vec::with_capacity(length.into());

        for _ in 0..length {
            annotations.push(self.type_annotation());
        }

        annotations
    }

    fn stackmapframe(&mut self) -> StackMapFrame {
        let frame_type = self.u1();

        match frame_type {
            0..=63 => StackMapFrame::SameFrame(SameFrame { frame_type }),
            64..=127 => StackMapFrame::SameLocals1StackItemFrame(SameLocals1StackItemFrame {
                frame_type,
                stack: self.verification_type_info(),
            }),
            247 => {
                let offset_delta = self.u2();
                let stack = self.verification_type_info();

                StackMapFrame::SameLocals1StackItemFrameExtended(
                    SameLocals1StackItemFrameExtended {
                        frame_type,
                        offset_delta,
                        stack,
                    },
                )
            }
            248..=250 => StackMapFrame::ChopFrame(ChopFrame {
                frame_type,
                offset_delta: self.u2(),
            }),
            251 => StackMapFrame::SameFrameExtended(SameFrameExtended {
                frame_type,
                offset_delta: self.u2(),
            }),
            252..=254 => {
                let offset_delta = self.u2();
                let length = frame_type - 251;
                let mut locals = Vec::with_capacity(length as usize);

                for _ in 0..length {
                    locals.push(self.verification_type_info());
                }

                StackMapFrame::AppendFrame(AppendFrame {
                    frame_type,
                    offset_delta,
                    locals,
                })
            }
            255 => {
                let offset_delta = self.u2();

                let length = self.u2();
                let mut locals = Vec::with_capacity(self.to_u2(length) as usize);

                for _ in 0..self.to_u2(length) {
                    locals.push(self.verification_type_info());
                }

                let length = self.u2();
                let mut stack = Vec::with_capacity(self.to_u2(length) as usize);

                for _ in 0..self.to_u2(length) {
                    stack.push(self.verification_type_info());
                }

                StackMapFrame::FullFrame(FullFrame {
                    frame_type,
                    offset_delta,
                    locals,
                    stack,
                })
            }
            _ => unreachable!(),
        }
    }

    fn verification_type_info(&mut self) -> VerificationTypeInfo {
        let tag = self.u1();

        match tag {
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
            _ => unreachable!(),
        }
    }

    fn attributes(&mut self, length: u16, cp: &Vec<CpNode>) -> Result<Vec<Attributes<'class>>> {
        let mut attributes = Vec::with_capacity(length as usize);

        while attributes.len() < length as usize {
            let attribute_name_index = self.u2();
            let attribute_length = self.u4();
            let tag = &cp[self.to_u2(attribute_name_index) as usize - 1];

            if let CpNode::Utf8(tag) = tag {
                match dbg!(tag.bytes) {
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
                        let mut requires = Vec::with_capacity(self.to_u2(requires_count).into());

                        for _ in 0..self.to_u2(requires_count) {
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
                        let mut exports = Vec::with_capacity(self.to_u2(exports_count).into());
                        for _ in 0..self.to_u2(exports_count) {
                            let exports_index = self.u2();
                            let exports_flags = self.u2();
                            let exports_to_count = self.u2();
                            let exports_to_index =
                                self.u2_range(self.to_u2(exports_to_count).into());

                            exports.push(ModuleExports {
                                exports_index,
                                exports_flags,
                                exports_to_index,
                            });
                        }

                        let opens_count = self.u2();
                        let mut opens = Vec::with_capacity(self.to_u2(opens_count).into());

                        for _ in 0..self.to_u2(opens_count) {
                            let opens_index = self.u2();
                            let opens_flags = self.u2();
                            let opens_to_count = self.u2();
                            let opens_to_index = self.u2_range(self.to_u2(opens_to_count).into());

                            opens.push(ModuleOpens {
                                opens_index,
                                opens_flags,
                                opens_to_index,
                            })
                        }

                        let uses_count = self.u2();
                        let uses_index = self.u2_range(self.to_u2(uses_count).into());

                        let provides_count = self.u2();
                        let mut provides = Vec::with_capacity(self.to_u2(provides_count).into());

                        for _ in 0..self.to_u2(provides_count) {
                            let provides_index = self.u2();
                            let provides_with_count = self.u2();
                            let provides_with_index =
                                self.u2_range(self.to_u2(provides_with_count).into());

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
                            Vec::with_capacity(self.to_u2(exception_table_length).into());

                        for _ in 0..self.to_u2(exception_table_length) {
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
                        let local_attributes = self.attributes(self.to_u2(attributes_count), cp)?;

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
                            Vec::with_capacity(self.to_u2(line_number_table_length).into());

                        for _ in 0..self.to_u2(line_number_table_length) {
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
                        let mut entries = Vec::with_capacity(self.to_u2(number_of_entries).into());

                        for _ in 0..self.to_u2(number_of_entries) {
                            entries.push(self.stackmapframe());
                        }

                        attributes.push(Attributes::StackMapTable(StackMapTable { entries }))
                    }

                    "Exceptions" => {
                        let number_of_exceptions = self.u2();
                        let exception_index_table =
                            self.u2_range(self.to_u2(number_of_exceptions).into());

                        attributes.push(Attributes::Exceptions(Exceptions {
                            exception_index_table,
                        }))
                    }

                    "InnerClasses" => {
                        let number_of_classes = self.u2();
                        let mut classes = Vec::with_capacity(self.to_u2(number_of_classes).into());

                        for _ in 0..self.to_u2(number_of_classes) {
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
                        let mut local_variable_table =
                            Vec::with_capacity(self.to_u2(length).into());

                        for _ in 0..self.to_u2(length) {
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
                            Vec::with_capacity(self.to_u2(length).into());

                        for _ in 0..self.to_u2(length) {
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
                        let annotations = self.annotation_range(self.to_u2(length));

                        attributes.push(Attributes::RuntimeVisibleAnnotations(
                            RuntimeVisibleAnnotations { annotations },
                        ))
                    }

                    "RuntimeInvisibleAnnotations" => {
                        println!("{}", self.index);
                        let length = self.u2();
                        let annotations = self.annotation_range(self.to_u2(length));

                        attributes.push(Attributes::RuntimeInvisibleAnnotations(
                            RuntimeInvisibleAnnotations { annotations },
                        ))
                    }

                    "RuntimeVisibleParameterAnnotations" => {
                        let length = self.u1();
                        let mut parameter_annotations = Vec::with_capacity(length.into());

                        for _ in 0..length {
                            let length = self.u2();
                            let annotations = self.annotation_range(self.to_u2(length));
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
                            let annotations = self.annotation_range(self.to_u2(length));
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

                        let annotations = self.type_annotation_range(self.to_u2(length));

                        attributes.push(Attributes::RuntimeVisibleTypeAnnotations(
                            RuntimeVisibleTypeAnnotations {
                                type_annotation: annotations,
                            },
                        ))
                    }

                    "RuntimeInvisibleTypeAnnotations" => {
                        let length = self.u2();
                        let annotations = self.type_annotation_range(self.to_u2(length));

                        attributes.push(Attributes::RuntimeInvisibleTypeAnnotations(
                            RuntimeInvisibleTypeAnnotations { annotations },
                        ))
                    }

                    "AnnotationDefault" => {
                        dbg!();
                        let default_value = self.element_value();

                        attributes.push(Attributes::AnnotationDefault(AnnotationDefault {
                            default_value,
                        }))
                    }

                    "BootstrapMethods" => {
                        let length = self.u2();
                        let mut bootstrap_methods = Vec::with_capacity(self.to_u2(length).into());

                        for _ in 0..self.to_u2(length) {
                            let bootstrap_method_ref = self.u2();
                            let num_bootstrap_arguments = self.u2();
                            let bootstrap_arguments =
                                self.u2_range(self.to_u2(num_bootstrap_arguments).into());

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
                        let package_index = self.u2_range(self.to_u2(package_count).into());

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
                        let classes = self.u2_range(self.to_u2(number_of_classes).into());
                        attributes.push(Attributes::NestMembers(NestMembers { classes }))
                    }

                    "Record" => {
                        let count = self.u2();
                        let mut components = Vec::with_capacity(self.to_u2(count).into());

                        for _ in 0..self.to_u2(count) {
                            let name_index = self.u2();
                            let descriptor_index = self.u2();
                            let attributes_count = self.u2();
                            let attributes = self.attributes(self.to_u2(attributes_count), cp)?;

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
                        let classes = self.u2_range(self.to_u2(length).into());

                        attributes.push(Attributes::PermittedSubclasses(PermittedSubclasses {
                            classes,
                        }))
                    }

                    _ => unimplemented!("{:?} {}", tag, attribute_length),
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
    ) -> Result<Vec<MethodInfo<'class>>> {
        let mut methods = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();
            let attributes = self.attributes(self.to_u2(attributes_count), cp)?;
            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            })
        }

        Ok(methods)
    }

    fn fields(&mut self, length: u16, cp: &Vec<CpNode<'class>>) -> Result<Vec<FieldInfo<'class>>> {
        let mut fields = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();
            let attributes = self.attributes(self.to_u2(attributes_count), cp)?;

            fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            })
        }

        Ok(fields)
    }

    pub fn parse(&mut self) -> Result<ClassFile<'class>> {
        let magic = self.u4();

        if magic != 0xCAFEBABE {
            return Err(ParsingError::Magic);
        }

        let minor_v = self.u2();
        let major_v = self.u2();

        let cp_count = self.u2();
        let cp = self.cp(self.to_u2(cp_count));

        let access_flags = self.u2();
        let this_class = self.u2();
        let super_class = self.u2();

        let interfaces_count = self.u2();
        let interfaces = self.u2_range(self.to_u2(interfaces_count) as u32);

        let fields_count = self.u2();
        let fields = self.fields(self.to_u2(fields_count), &cp)?;

        let methods_count = self.u2();
        let methods = self.methods(self.to_u2(methods_count), &cp)?;

        let attributes_count = self.u2();
        let attributes = self.attributes(self.to_u2(attributes_count), &cp)?;

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
