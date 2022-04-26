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
    Utf8(&'class [u8]),
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
pub struct Parser {
    pub bytes: Vec<u8>,
    pub index: usize,
}

impl Parser {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, index: 0 }
    }

    pub fn u1(&mut self) -> U1 {
        let output = self.bytes[self.index];
        self.index += 1;
        output
    }

    pub fn u2(&mut self) -> U2 {
        let output = U2::from_be_bytes(self.bytes[self.index..self.index + 2].try_into().unwrap());
        self.index += 2;
        output
    }

    pub fn u4(&mut self) -> U4 {
        let output = U4::from_be_bytes(self.bytes[self.index..self.index + 4].try_into().unwrap());
        self.index += 4;
        output
    }

    pub fn u1_range(&mut self, end: usize) -> &[u8] {
        let output = &self.bytes[self.index..self.index + end];
        self.index += end;
        output
    }

    pub fn parse_child_pool(&mut self) -> Result<Vec<CpNode>> {
        let length = self.u2();
        let mut cp: Vec<CpNode> = Vec::with_capacity((length - 1).into());

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

    pub fn parse_attributes(&mut self, length: U2) -> Vec<AttributeInfo> {
        let mut attributes = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let attribute_name_index = self.u2();
            let len = self.u4();

            attributes.push(AttributeInfo {
                attribute_name_index,
                info: self.u1_range(len as usize),
            });
        }

        attributes
    }

    pub fn parse<'parse>(&'parse mut self) -> Result<ClassFile> {
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

            fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes: self.parse_attributes(attributes_count),
            })
        }

        let methods_length = self.u2();
        let mut methods = Vec::with_capacity(methods_length as usize);
        for _ in 0..methods_length {
            let access_flags = self.u2();
            let name_index = self.u2();
            let descriptor_index = self.u2();
            let attributes_count = self.u2();

            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes: self.parse_attributes(attributes_count),
            })
        }

        let attributes_length = self.u2();

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
            attributes: self.parse_attributes(attributes_length),
        })
    }
}
