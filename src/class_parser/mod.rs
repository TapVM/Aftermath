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
    pub index: usize,
}

impl<'class> Parser<'class> {
    pub fn new(bytes: &'class [U1]) -> Self {
        Self { bytes, index: 0 }
    }

    pub fn u1(&mut self) -> U1 {
        let output = self.bytes[self.index];
        self.index += 1;
        output
    }

    pub fn u1_range(&mut self, end: usize) -> &'class [U1] {
        let output = &self.bytes[self.index..self.index + end];
        self.index += end;
        output
    }

    pub fn u2(&mut self) -> U2 {
        U2::from_be_bytes(self.u1_range(2).try_into().unwrap())
    }

    pub fn u4(&mut self) -> U4 {
        U4::from_be_bytes(self.u1_range(4).try_into().unwrap())
    }

    pub fn has_flag(&mut self, integer: U2, flag: U2) -> bool {
        integer & flag != 0
    }

    pub fn parse_child_pool(&mut self) -> Result<Vec<CpNode<'class>>> {
        let length = self.u2();
        let mut cp: Vec<CpNode<'class>> = Vec::with_capacity((length - 1).into());

        for _ in 0..length - 1 {
            let tag = self.u1();

            match tag {
                7 => {
                    cp.push(CpNode::Class(self.u2()));
                }

                9 => {
                    cp.push(CpNode::FieldRef(self.u2(), self.u2()));
                }

                10 => {
                    cp.push(CpNode::MethodRef(self.u2(), self.u2()));
                }

                11 => {
                    cp.push(CpNode::InterfaceMethodRef(self.u2(), self.u2()));
                }

                8 => {
                    cp.push(CpNode::String(self.u2()));
                }

                3 => {
                    cp.push(CpNode::Integer(self.u4()));
                }

                4 => {
                    cp.push(CpNode::Float(self.u4()));
                }

                5 => {
                    cp.push(CpNode::Long(self.u4(), self.u4()));
                }

                6 => {
                    cp.push(CpNode::Double(self.u4(), self.u4()));
                }

                12 => {
                    cp.push(CpNode::NameAndType(self.u2(), self.u2()));
                }

                1 => {
                    let length = self.u2().into();
                    cp.push(CpNode::Utf8(std::str::from_utf8(self.u1_range(length))?));
                }

                15 => {
                    cp.push(CpNode::MethodHandle(self.u1(), self.u2()));
                }

                16 => {
                    cp.push(CpNode::MethodType(self.u2()));
                }

                17 => {
                    cp.push(CpNode::Dynamic(self.u2(), self.u2()));
                }

                18 => {
                    cp.push(CpNode::InvokeDynamic(self.u2(), self.u2()));
                }

                19 => {
                    cp.push(CpNode::Module(self.u2()));
                }

                20 => {
                    cp.push(CpNode::Package(self.u2()));
                }

                _ => {
                    return Err(ParsingError::ConstantPoolTag(tag));
                }
            }
        }

        Ok(cp)
    }

    pub fn assert_attr_length(&self, length: U2, expected: U2) -> Result<()> {
        if length != expected {
            return Err(ParsingError::AttributeLength(expected, length));
        }

        Ok(())
    }

    pub fn parse_element_value(&mut self) -> ElementValue {
        let element_name_index = self.u2();
        let value: ElementValueUnion;
        let tag = self.u1();
        let value_tag = tag as char;

        match value_tag {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                value = ElementValueUnion::ConstValueIndex(self.u2())
            }

            'e' => {
                value = ElementValueUnion::EnumConstValue(EnumConstValue {
                    type_name_index: self.u2(),
                    const_name_index: self.u2(),
                })
            }

            'c' => value = ElementValueUnion::ClassInfoIndex(self.u2()),

            '@' => value = ElementValueUnion::AnnotationValue(self.parse_annotations()),

            '[' => {
                let length = self.u2();
                let mut values = Vec::new();

                for _ in 0..length {
                    values.push(self.parse_element_value());
                }

                value = ElementValueUnion::ArrayValue(ArrayValue {
                    element_value: values,
                })
            }

            _ => unreachable!(),
        }

        ElementValue { tag, value }
    }

    pub fn parse_annotations(&mut self) -> Annotation {
        let type_index = self.u2();
        let num_element_variable_pairs = self.u2();
        let mut element_value_pairs = Vec::new();

        for _ in 0..num_element_variable_pairs {
            element_value_pairs.push(AnnotationInner {
                element_name_index: self.u2(),
                value: self.parse_element_value(),
            })
        }

        Annotation {
            type_index,
            element_value_pairs,
        }
    }

    pub fn parse_attributes(
        &mut self,
        length: U2,
        constant_pool: &[CpNode],
        is_module: bool,
    ) -> Result<Vec<Attributes<'class>>> {
        let mut attributes: Vec<Attributes> = Vec::new();

        for _ in 0..length {
            let attribute_name_index = self.u2();
            let attribute_length = self.u2();

            if let Some(data) = constant_pool.get(attribute_name_index as usize + 1) {
                if let CpNode::Utf8(data) = data {
                    // -----------------------------------------------------------------------------

                    match *data {
                        "ConstantValue" => {
                            let attribute_length = self.u2();
                            self.assert_attr_length(attribute_length, 2)?;

                            let constantvalue_index = self.u2();
                            if let Some(data) = constant_pool.get(constantvalue_index as usize) {
                                match data {
                                    &CpNode::Integer(..)
                                    | &CpNode::String(..)
                                    | &CpNode::Long(..)
                                    | &CpNode::Double(..)
                                    | &CpNode::Float(..) => {}
                                    _ => return Err(ParsingError::IllegalValueAttribute),
                                }
                            } else {
                                return Err(ParsingError::AttributeIndex(constantvalue_index));
                            }

                            attributes.push(Attributes::Value(Value {
                                value_index: constantvalue_index,
                            }))
                        }

                        "Code" => {
                            let max_stack = self.u2();
                            let max_locals = self.u2();
                            let code_length = self.u4();
                            let code = self.u1_range(code_length as usize);

                            let exception_table_length = self.u2();
                            let mut exception_table: Vec<ExceptionTableAttrCode> = Vec::new();

                            for _ in 0..exception_table_length {
                                exception_table.push(ExceptionTableAttrCode {
                                    start_pc: self.u2(),
                                    end_pc: self.u2(),
                                    handler_pc: self.u2(),
                                    catch_type: self.u2(),
                                });
                            }

                            let attributes_count = self.u2();
                            let local_attributes =
                                self.parse_attributes(attributes_count, constant_pool, is_module)?;

                            attributes.push(Attributes::Code(AttrCode {
                                max_stack,
                                max_locals,
                                code,
                                exception_table,
                                attributes: local_attributes,
                            }))
                        }

                        "StackMapTable" => {
                            let number_of_entries = self.u2();
                            let entries = self.u1_range(number_of_entries as usize);

                            attributes.push(Attributes::StackMapTable(StackMapTable { entries }))
                        }

                        "Exceptions" => {
                            let number_of_exceptions = self.u2();
                            let mut exception_index_table: Vec<U2> = Vec::new();

                            for _ in 0..number_of_exceptions {
                                exception_index_table.push(self.u2());
                            }

                            attributes.push(Attributes::Exceptions(Exceptions {
                                exception_index_table,
                            }))
                        }

                        "InnerClasses" => {
                            let number_of_classes = self.u2();
                            let mut classes: Vec<ClassesInnerClassAttr> = Vec::new();

                            for _ in 0..number_of_classes {
                                classes.push(ClassesInnerClassAttr {
                                    inner_class_info_index: self.u2(),
                                    outer_class_info_index: self.u2(),
                                    inner_name_index: self.u2(),
                                    inner_class_access_flags: self.u2(),
                                })
                            }

                            attributes.push(Attributes::InnerClass(InnerClass { classes }))
                        }

                        "EnclosingMethod" => {
                            attributes.push(Attributes::EnclosingMethod(EnclosingMethod {
                                class_index: self.u2(),
                                method_index: self.u2(),
                            }))
                        }

                        "Synthetic" => attributes.push(Attributes::Synthetic(Synthetic)),

                        "Signature" => attributes.push(Attributes::Signature(Signature {
                            signature_index: self.u2(),
                        })),

                        "SourceFile" => attributes.push(Attributes::SourceFile(SourceFile {
                            sourcefile_index: self.u2(),
                        })),

                        "SourceDebugExtension" => {
                            attributes.push(Attributes::SourceDebugExt(SourceDebugExt {
                                debug_extension: self.u1_range(attribute_length as usize),
                            }))
                        }

                        "LineNumberTable" => {
                            let length = self.u2();
                            let mut line_number_table = Vec::new();

                            for _ in 0..length {
                                line_number_table.push(LineNumberTableAttrInner {
                                    start_pc: self.u2(),
                                    line_number: self.u2(),
                                })
                            }

                            attributes.push(Attributes::LineNumberTable(LineNumberTable {
                                line_number_table,
                            }))
                        }

                        "LocalVariableTable" => {
                            let length = self.u2();
                            let mut local_variable_table = Vec::new();

                            for _ in 0..length {
                                local_variable_table.push(LocalVariableTableAttrInner {
                                    start_pc: self.u2(),
                                    length: self.u2(),
                                    name_index: self.u2(),
                                    descriptor_index: self.u2(),
                                    index: self.u2(),
                                });
                            }

                            attributes.push(Attributes::LocalVariableTable(LocalVariableTable {
                                local_variable_table,
                            }))
                        }

                        "LocalVariableTypeTable" => {
                            let length = self.u2();
                            let mut local_variable_type_table = Vec::new();

                            for _ in 0..length {
                                local_variable_type_table.push(LocalVariableTypeTableAttrInner {
                                    start_pc: self.u2(),
                                    length: self.u2(),
                                    name_index: self.u2(),
                                    signature_index: self.u2(),
                                    index: self.u2(),
                                });
                            }

                            attributes.push(Attributes::LocalVariableTypeTable(
                                LocalVariableTypeTable {
                                    local_variable_type_table,
                                },
                            ))
                        }

                        "RuntimeVisibleAnnotations" => {
                            let length = self.u2();
                            let mut annotations: Vec<Annotation> = Vec::new();

                            for _ in 0..length {}
                        }

                        _ => todo!(),
                    }
                } else {
                    return Err(ParsingError::AttributeNotUtf8);
                }
            } else {
                return Err(ParsingError::AttributeIndex(attribute_name_index));
            }
        }

        Ok(attributes)
    }

    pub fn parse(&mut self) -> Result<ClassFile<'class>> {
        let magic = self.u4();

        if magic != 0xCAFEBABE {
            return Err(ParsingError::Magic);
        }

        let minor_v = self.u2();
        let major_v = self.u2();

        if !(45..=61).contains(&major_v) {
            return Err(ParsingError::MajorVersion);
        }

        if 56 <= major_v && !(minor_v == 0 || minor_v == 65535) {
            return Err(ParsingError::MinorVersion);
        }

        let cp: Vec<CpNode<'class>> = self.parse_child_pool()?;
        let access_flags = self.u2();
        let mut is_module = false;

        if self.has_flag(access_flags, 0x8000) {
            if access_flags != 0x8000 {
                return Err(ParsingError::ContainsOtherFlagsWhileBeingAModule);
            }

            if major_v < 53 {
                return Err(ParsingError::InvalidVersionForModule);
            }

            is_module = true;
        } else {
            if self.has_flag(access_flags, 0x0200) {
                if !self.has_flag(access_flags, 0x0400) {
                    return Err(ParsingError::InterfaceWithoutAbstract);
                }

                if self.has_flag(access_flags, 0x0010)
                    || self.has_flag(access_flags, 0x0020)
                    || self.has_flag(access_flags, 0x4000)
                    || self.has_flag(access_flags, 0x8000)
                {
                    return Err(ParsingError::ContainsIllegalFlagsAsInterface);
                }
            } else {
                if self.has_flag(access_flags, 0x2000) || self.has_flag(access_flags, 0x8000) {
                    return Err(ParsingError::ContainsIllegalFlagsAsNonInterface);
                }

                if self.has_flag(access_flags, 0x0010) && self.has_flag(access_flags, 0x0400) {
                    return Err(ParsingError::ContainsFinalAndAbstractAsNonInterface);
                }
            }

            if self.has_flag(access_flags, 0x2000) && !self.has_flag(access_flags, 0x0200) {
                return Err(ParsingError::AnnotationWithoutInterface);
            }
        }

        let this_class = self.u2();
        let super_class = self.u2();
        let interfaces_length = self.u2();

        let mut interfaces = Vec::with_capacity(interfaces_length as usize);
        for _ in 0..interfaces_length {
            interfaces.push(self.u2());
        }

        let fields_length = self.u2();
        let mut fields = Vec::with_capacity(fields_length as usize);
        for _ in 0..fields_length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();
            let attributes = self.parse_attributes(attributes_count, &cp, false)?;

            fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            })
        }

        let methods_length = self.u2();
        let mut methods = Vec::with_capacity(methods_length as usize);

        for _ in 0..methods_length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();
            let attributes = self.parse_attributes(attributes_count, &cp, false)?;

            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            })
        }

        let attributes_length = self.u2();
        let attributes = if is_module {
            if super_class != 0
                || interfaces_length != 0
                || fields_length != 0
                || methods_length != 0
            {
                return Err(ParsingError::ModuleHasIllegalVariables);
            }

            self.parse_attributes(attributes_length, &cp, true)?
        } else {
            self.parse_attributes(attributes_length, &cp, false)?
        };

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
