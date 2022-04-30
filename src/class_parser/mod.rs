#![allow(dead_code)]

mod errors;
pub use errors::ParsingError;

type U1 = u8;
type U2 = u16;
type U4 = u32;
type Result<T, E = ParsingError> = core::result::Result<T, E>;

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
    Utf8(&'class [U1]),
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
    BootstrapMethods(BootstrapMethods),
    MethodParameters(MethodParameters),
    Module(Module),
    ModulePackages(ModulePackages),
    ModuleMainClass(ModuleMainClass),
    NestHost(NestHost),
    NestMembers(NestMembers),
    Record(Record),
    PermittedSubclasses(PermittedSubclasses),
}

#[derive(Debug)]
pub struct LineNumberTableAttrInner {
    start_pc: U2,
    line_number: U2,
}

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
pub struct Value {
    value_index: U2,
}

#[derive(Debug)]
pub struct Exceptions {
    exception_index_table: Vec<U2>,
}

#[derive(Debug)]
pub struct AttributeInfo<'class> {
    pub attribute_name_index: U2,
    pub info: &'class [U1],
}

#[derive(Debug)]
pub struct FieldInfo<'class> {
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes: Vec<AttributeInfo<'class>>,
}

#[derive(Debug)]
pub struct MethodInfo<'class> {
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes: Vec<AttributeInfo<'class>>,
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
    pub attributes: Vec<AttributeInfo<'class>>,
}

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
                    cp.push(CpNode::Utf8(self.u1_range(length)));
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

    pub fn parse_attributes(
        &mut self,
        length: U2,
        is_module: bool,
    ) -> Result<Vec<AttributeInfo<'class>>> {
        let mut attributes = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let attribute_name_index = self.u2();
            if is_module {}
            let len = self.u4();
            let info = self.u1_range(len as usize);

            attributes.push(AttributeInfo {
                attribute_name_index,
                info,
            });
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

        let cp = self.parse_child_pool()?;
        let access_flags = self.u2();
        let mut is_module = false;

        if self.has_flag(access_flags, 0x8000) {
            if access_flags != 0x8000 {
                return Err(ParsingError::ContainsOtherFlagsWhileBeingAModule);
            }

            if !(major_v >= 53) {
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
            let attributes = self.parse_attributes(attributes_count, false)?;

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
            let attributes = self.parse_attributes(attributes_count, false)?;

            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            })
        }

        let attributes_length = self.u2();
        let attributes: Vec<AttributeInfo>;

        if is_module {
            if super_class != 0
                || interfaces_length != 0
                || fields_length != 0
                || methods_length != 0
            {
                return Err(ParsingError::ModuleHasIllegalVariables);
            }

            attributes = self.parse_attributes(attributes_length, true)?;
        } else {
            attributes = self.parse_attributes(attributes_length, false)?;
        }

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
