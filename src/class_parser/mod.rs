#![allow(dead_code)]

mod errors;

pub use errors::ParsingError;

type U1 = u8;
type U2 = u16;
type U4 = u32;
type Result<T, E = ParsingError> = core::result::Result<T, E>;

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum CpNode<'class> {
    Class(U2),
    String(U2),
    MethodType(U2),
    Module(U2),
    Package(U2),
    Integer(U4),
    Float(U4),
    Dynamic(U2, U2),
    NameAndType(U2, U2),
    InvokeDynamic(U2, U2),
    FieldRef(U2, U2),
    MethodRef(U2, U2),
    InterfaceMethodRef(U2, U2),
    Long(U4, U4),
    Double(U4, U4),
    MethodHandle(U1, U2),
    Utf8(&'class str),
}

#[derive(Debug)]
pub enum Attributes<'class> {
    Value(Value),
    Code(AttrCode<'class>),
    StackMapTable(StackMapTable<'class>),
    Exceptions(Exceptions),
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
    BootstrapMethods(BootStrapMethods),
    MethodParameters(MethodParameters),
    Module(Module),
    ModulePackages(ModulePackages),
    ModuleMainClass(ModuleMainClass),
    NestHost(NestHost),
    NestMembers(NestMembers),
    Record(Record<'class>),
    PermittedSubclasses(PermittedSubclasses),
}

#[derive(Debug)]
pub enum TargetInfo {
    TypeParameter,
    Supertype,
    TypeParameterBound,
    Empty,
    FormalParameter,
    Throws,
    Localvar,
    Catch,
    Offset,
    TypeArgument,
}

#[derive(Debug)]
pub enum ElementValueUnion {
    ConstValueIndex(U2),
    EnumConstValue(EnumConstValue),
    ClassInfoIndex(U2),
    AnnotationValue(Annotation),
    ArrayValue(ArrayValue),
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PermittedSubclasses {
    classes: Vec<U2>,
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
pub struct ModulePackages {
    package_index: Vec<U2>,
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
pub struct NestMembers {
    classes: Vec<U2>,
}

#[derive(Debug)]
pub struct ModuleRequires {
    requires_index: U2,
    requires_flags: U2,
    require_version_index: U2,
}

#[derive(Debug)]
pub struct ModuleExports {
    exports_index: U2,
    exports_flags: U2,
    exports_to_index: Vec<U2>,
}

#[derive(Debug)]
pub struct ModuleOpens {
    opens_index: U2,
    opens_flags: U2,
    opens_to_index: Vec<U2>,
}

#[derive(Debug)]
pub struct ModuleProvides {
    provides_index: U2,
    provides_with_index: Vec<U2>,
}

#[derive(Debug)]
pub struct Module {
    module_name_index: U2,
    module_flags: U2,
    module_version_index: U2,
    requires: Vec<ModuleRequires>,
    exports: Vec<ModuleExports>,
    opens: Vec<ModuleOpens>,
    uses_index: Vec<U2>,
    provides: Vec<ModuleProvides>,
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
pub struct ElementValue {
    tag: U1,
    value: ElementValueUnion,
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
pub struct StackMapTable<'class> {
    entries: &'class [U1],
}

#[derive(Debug)]
pub struct BootStrapMethodsInner {
    bootstrap_method_ref: U2,
    bootstrap_arguments: Vec<U2>,
}

#[derive(Debug)]
pub struct BootStrapMethods {
    bootstrap_methods: Vec<BootStrapMethodsInner>,
}

#[derive(Debug)]
pub struct Value {
    value_index: U2,
}

#[derive(Debug)]
pub struct Exceptions {
    exception_index_table: Vec<U2>,
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
    pub interfaces: Vec<U2>,
    pub fields: Vec<FieldInfo<'class>>,
    pub methods: Vec<MethodInfo<'class>>,
    pub attributes: Vec<Attributes<'class>>,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Parser<'class> {
    pub bytes: &'class [U1],
}

impl<'class> Parser<'class> {
    pub fn new(bytes: &'class [u8]) -> Self {
        Self { bytes }
    }

    pub fn u1(&mut self) -> U1 {
        let output = self.bytes[0];
        self.bytes = &self.bytes[1..];
        output
    }

    pub fn u1_range(&mut self, length: U4) -> &'class [U1] {
        let output = &self.bytes[0..length as usize];
        self.bytes = &self.bytes[length as usize..];
        output
    }

    pub fn u2(&mut self) -> U2 {
        U2::from_be_bytes(self.u1_range(2).try_into().unwrap())
    }

    pub fn u2_range(&mut self, length: U4) -> Vec<U2> {
        let mut bytes = Vec::new();

        for _ in 0..length {
            bytes.push(self.u2());
        }

        bytes
    }

    pub fn u4(&mut self) -> U4 {
        U4::from_be_bytes(self.u1_range(4).try_into().unwrap())
    }

    pub fn cp(&mut self, length: U2) -> Result<Vec<CpNode<'class>>> {
        let mut pool = Vec::with_capacity(length as usize - 1);
        for _ in 0..(length - 1_u16) {
            let tag = self.u1();

            match tag {
                7 => pool.push(CpNode::Class(self.u2())),
                9 => pool.push(CpNode::FieldRef(self.u2(), self.u2())),
                10 => pool.push(CpNode::FieldRef(self.u2(), self.u2())),
                11 => pool.push(CpNode::MethodRef(self.u2(), self.u2())),
                8 => pool.push(CpNode::String(self.u2())),
                3 => pool.push(CpNode::Integer(self.u4())),
                5 => pool.push(CpNode::Long(self.u4(), self.u4())),
                6 => pool.push(CpNode::Double(self.u4(), self.u4())),
                12 => pool.push(CpNode::NameAndType(self.u2(), self.u2())),
                1 => {
                    let length = self.u2().into();
                    pool.push(CpNode::Utf8(std::str::from_utf8(self.u1_range(length))?));
                }
                15 => pool.push(CpNode::MethodHandle(self.u1(), self.u2())),
                16 => pool.push(CpNode::MethodType(self.u2())),
                17 => pool.push(CpNode::Dynamic(self.u2(), self.u2())),
                18 => pool.push(CpNode::InvokeDynamic(self.u2(), self.u2())),
                19 => pool.push(CpNode::Module(self.u2())),
                20 => pool.push(CpNode::Package(self.u2())),
                _ => return Err(ParsingError::ConstantPoolTag(tag)),
            }
        }

        Ok(pool)
    }

    pub fn attributes(&mut self, length: U2, cp: &Vec<CpNode>) -> Result<Vec<Attributes<'class>>> {
        let mut attributes = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let attribute_name_index = self.u2();
            let attribute_length = self.u4();
            let tag = &cp[attribute_name_index as usize - 1];

            if let CpNode::Utf8(tag) = tag {
                match *tag {
                    "SourceFile" => attributes.push(Attributes::SourceFile(SourceFile {
                        sourcefile_index: self.u2(),
                    })),

                    "Module" => {
                        let module_name_index = self.u2();
                        let module_flags = self.u2();
                        let module_version_index = self.u2();

                        let requires_count = self.u2();
                        let mut requires = Vec::new();

                        for _ in 0..requires_count {
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
                        let mut exports = Vec::new();
                        for _ in 0..exports_count {
                            let exports_index = self.u2();
                            let exports_flags = self.u2();
                            let exports_to_count = self.u2();
                            let mut exports_to_index = Vec::new();

                            for _ in 0..exports_to_count {
                                exports_to_index.push(self.u2());
                            }

                            exports.push(ModuleExports {
                                exports_index,
                                exports_flags,
                                exports_to_index,
                            });
                        }

                        let opens_count = self.u2();
                        let mut opens = Vec::new();

                        for _ in 0..opens_count {
                            let opens_index = self.u2();
                            let opens_flags = self.u2();
                            let opens_to_count = self.u2();
                            let mut opens_to_index = Vec::new();

                            for _ in 0..opens_to_count {
                                opens_to_index.push(self.u2());
                            }

                            opens.push(ModuleOpens {
                                opens_index,
                                opens_flags,
                                opens_to_index,
                            })
                        }

                        let uses_count = self.u2();
                        let mut uses_index = Vec::new();

                        for _ in 0..uses_count {
                            uses_index.push(self.u2());
                        }

                        let provides_count = self.u2();
                        let mut provides = Vec::new();

                        for _ in 0..provides_count {
                            let provides_index = self.u2();
                            let provides_with_count = self.u2();
                            let mut provides_with_index = Vec::new();

                            for _ in 0..provides_with_count {
                                provides_with_index.push(self.u2());
                            }

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
                        let mut exception_table = Vec::new();

                        for _ in 0..exception_table_length {
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
                        let local_attributes = self.attributes(attributes_count, cp)?;

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
                        let mut line_number_table = Vec::new();

                        for _ in 0..line_number_table_length {
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
                        let entries = self.u1_range(number_of_entries.into());

                        attributes.push(Attributes::StackMapTable(StackMapTable { entries }))
                    }

                    _ => todo!("{} {}", tag, attribute_length),
                }
            } else {
                return Err(ParsingError::AttributeNotUtf8);
            }
        }

        Ok(attributes)
    }

    pub fn methods(&mut self, length: U2, cp: &Vec<CpNode>) -> Result<Vec<MethodInfo>> {
        let mut methods = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();
            let attributes = self.attributes(attributes_count, cp)?;
            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            })
        }

        Ok(methods)
    }

    pub fn fields(&mut self, length: U2, cp: &Vec<CpNode>) -> Result<Vec<FieldInfo>> {
        let mut fields = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();
            let attributes = self.attributes(attributes_count, &cp)?;

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

        let buffer = self.u2_range(3);
        let minor_v = buffer[0];
        let major_v = buffer[1];
        let cp_count = buffer[2];
        let cp = self.cp(cp_count)?;

        let buffer = self.u2_range(4);
        let access_flags = buffer[0];
        let this_class = buffer[1];
        let super_class = buffer[2];

        let interfaces_count = buffer[3];
        let interfaces = self.u2_range(interfaces_count as u32);
        let fields_count = self.u2();
        let fields = self.fields(fields_count, &cp)?;
        let methods_count = self.u2();
        let methods = self.methods(methods_count, &cp)?;
        let attributes_count = self.u2();
        let attributes = self.attributes(attributes_count, &cp)?;

        Ok(ClassFile {
            minor_v: 1,
            major_v: 1,
            cp: Vec::new(),
            access_flags: 1,
            this_class: 1,
            super_class: 1,
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
        })
    }
}
