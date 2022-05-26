use super::{Attributes, BootStrapMethods, ClassFile, CpNode, ParsingError};

pub struct Verifier<'a> {
    class: &'a ClassFile<'a>,
    bootstrap_methods: Vec<&'a BootStrapMethods<'a>>,
}

impl<'a> Verifier<'a> {
    pub fn new(class: &'a ClassFile<'a>) -> Self {
        let bootstrap_methods = class
            .attributes
            .iter()
            .filter(|x| matches!(x, Attributes::BootstrapMethods(..)))
            .map(|y| match y {
                Attributes::BootstrapMethods(z) => z,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        Self {
            class,
            bootstrap_methods,
        }
    }

    pub fn verify(self) -> Result<&'a ClassFile<'a>, ParsingError> {
        let class = self.class;

        let major_v = class.major_v.to_u2();
        let minor_v = class.minor_v.to_u2();

        if !(45..=61).contains(&major_v) {
            return Err(ParsingError::InvalidMajorV);
        }

        if major_v >= 56 && (minor_v != 0 || minor_v != 65535) {
            return Err(ParsingError::InvalidMinorV);
        }

        for node in &class.cp {
            self.verify_cp_node(node)?;
        }

        let this_class = match match &class.cp[class.this_class.to_u2() as usize - 1] {
            CpNode::Class(z) => &class.cp[z.name_index.to_u2() as usize - 1],
            _ => unreachable!(),
        } {
            CpNode::Utf8(z) => z.bytes,
            _ => unreachable!(),
        };

        let access_flags = class.access_flags.to_u2();

        if access_flags & 0x8000 == 0 {
            if access_flags & 0x0200 != 0 {
                if access_flags & 0x0400 == 0 {
                    return Err(ParsingError::InterfaceWithoutAbstract);
                }

                if access_flags & 0x0010 != 0
                    || access_flags & 0x0020 != 0
                    || access_flags & 0x4000 != 0
                    || access_flags & 0x8000 != 0
                {
                    return Err(ParsingError::IllegalFlagsWhileInterface);
                }
            } else {
                if access_flags & 0x2000 != 0 || access_flags & 0x8000 != 0 {
                    return Err(ParsingError::IllegalFlagsWhileNonInterface);
                }

                if access_flags & 0x0400 != 0 && access_flags & 0x0010 != 0 {
                    return Err(ParsingError::FinalAndAbstractWhileNonInterface);
                }
            }

            if access_flags & 0x2000 != 0 && access_flags & 0x0200 == 0 {
                return Err(ParsingError::AnnotationWithoutInterface);
            }
        } else {
            // ACC_MODULE is set

            if major_v < 53 {
                return Err(ParsingError::InvalidVersionAsModule);
            }

            if this_class != "module-info" {
                return Err(ParsingError::ThisClassNotModuleInfoAsModule);
            }

            if class.super_class.to_u2() != 0
                || !class.interfaces.is_empty()
                || !class.fields.is_empty()
                || !class.methods.is_empty()
            {
                return Err(ParsingError::VarsNotZeroAsModule);
            }

            // TODO
            /*
                • attributes: One Module attribute must be present. Except
                for Module, ModulePackages, ModuleMainClass, InnerClasses,
                SourceFile, SourceDebugExtension, RuntimeVisibleAnnotations, and
                RuntimeInvisibleAnnotations, none of the pre-defined attributes (§4.7) may
                appear.
            */
        }

        Ok(class)
    }

    fn verify_cp_node(&self, node: &CpNode) -> Result<(), ParsingError> {
        let cp = &self.class.cp;
        let bootstrap_methods = &self.bootstrap_methods;

        match node {
            CpNode::Class(class) => {
                let node = &cp[class.name_index.to_u2() as usize - 1];
                match node {
                    CpNode::Utf8(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::ClassNodeNotPointingToUtf8),
                };
            }
            CpNode::FieldRef(fieldref) => {
                let class_index = fieldref.class_index;
                let name_and_type = fieldref.name_and_type_index;

                let node = &cp[class_index.to_u2() as usize - 1];
                match node {
                    CpNode::Class(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::FieldRefNodeNotPointingToClass),
                };

                let node = &cp[name_and_type.to_u2() as usize - 1];
                match node {
                    CpNode::NameAndType(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::FieldRefNodeNotPointingToNameAndType),
                }
            }
            CpNode::MethodRef(methodref) => {
                let class_index = methodref.class_index;
                let name_and_type = methodref.name_and_type_index;

                let node = &cp[class_index.to_u2() as usize - 1];
                match node {
                    CpNode::Class(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::MethodRefNodeNotPointingToClass),
                };

                let node = &cp[name_and_type.to_u2() as usize - 1];
                match node {
                    CpNode::NameAndType(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::MethodRefNodeNotPointingToNameAndType),
                }
            }
            CpNode::InterfaceMethodRef(interfacemethodref) => {
                let class_index = interfacemethodref.class_index;
                let name_and_type = interfacemethodref.name_and_type_index;

                let node = &cp[class_index.to_u2() as usize - 1];
                match node {
                    CpNode::Class(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::InterfaceMethodRefNodeNotPointingToClass),
                };

                let node = &cp[name_and_type.to_u2() as usize - 1];
                match node {
                    CpNode::NameAndType(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::InterfaceMethodRefNodeNotPointingToNameAndType),
                }
            }
            CpNode::String(string) => {
                let node = &cp[string.string_index.to_u2() as usize - 1];
                match node {
                    CpNode::Utf8(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::StringNodeNotPointingToUtf8),
                };
            }
            CpNode::MethodType(methodtype) => {
                let node = &cp[methodtype.descriptor_index.to_u2() as usize - 1];
                match node {
                    CpNode::Utf8(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::MethodTypeNodeNotPointingToUtf8),
                };
            }
            CpNode::Module(module) => {
                let node = &cp[module.name_index.to_u2() as usize - 1];
                match node {
                    CpNode::Utf8(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::ModuleNodeNotPointingToUtf8),
                };
            }
            CpNode::Package(package) => {
                let package = &cp[package.name_index.to_u2() as usize - 1];
                match package {
                    CpNode::Utf8(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::PackageNodeNotPointingToUtf8),
                };
            }
            CpNode::Dynamic(dynamic) => {
                let bootstrap_method_attr_index = dynamic.bootstrap_method_attr_index.to_u2();

                if bootstrap_methods.len() != 1 {
                    return Err(ParsingError::InvalidAmountOfBootStrapMethodsInClass);
                } else if bootstrap_methods[0]
                    .bootstrap_methods
                    .get(bootstrap_method_attr_index as usize)
                    .is_none()
                {
                    return Err(
                        ParsingError::BootstrapMethodAttrIndexInDynamicAttributeIsNotValidIndex,
                    );
                }

                let name_and_type_index = dynamic.name_and_type_index.to_u2();
                let node = &cp[name_and_type_index as usize - 1];
                match node {
                    CpNode::NameAndType(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::DynamicNotPointingToNameAndType),
                };
            }
            CpNode::NameAndType(nameandtype) => {
                let name_index = nameandtype.name_index.to_u2();

                let node = &cp[name_index as usize - 1];
                match node {
                    CpNode::Utf8(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::PackageNodeNotPointingToUtf8),
                };
            }
            CpNode::InvokeDynamic(dynamic) => {
                let bootstrap_method_attr_index = dynamic.bootstrap_method_attr_index.to_u2();

                if bootstrap_methods.len() != 1 {
                    return Err(ParsingError::InvalidAmountOfBootStrapMethodsInClass);
                } else if bootstrap_methods[0]
                    .bootstrap_methods
                    .get(bootstrap_method_attr_index as usize)
                    .is_none()
                {
                    return Err(
                            ParsingError::BootstrapMethodAttrIndexInInvokeDynamicAttributeIsNotValidIndex,
                        );
                }

                let name_and_type_index = dynamic.name_and_type_index.to_u2();
                let node = &cp[name_and_type_index as usize - 1];
                match node {
                    CpNode::NameAndType(_) => self.verify_cp_node(node)?,
                    _ => return Err(ParsingError::InvokeDynamicNotPointingToNameAndType),
                };
            }
            CpNode::MethodHandle(methodhandle) => {
                let reference_kind = &methodhandle.reference_kind;
                let reference_index = methodhandle.reference_index.to_u2();

                if !(1..=9).contains(reference_kind) {
                    return Err(ParsingError::InvalidReferenceKind);
                } else if (1..=4).contains(reference_kind) {
                    let node = &cp[reference_index as usize - 1];
                    match node {
                        CpNode::FieldRef(_) => self.verify_cp_node(node)?,
                        _ => return Err(ParsingError::MethodHandle1to4NotPointingToFieldRef),
                    };
                } else if reference_kind == &5 || reference_kind == &8 {
                    let node = &cp[reference_index as usize - 1];
                    match node {
                        CpNode::MethodRef(_) => self.verify_cp_node(node)?,
                        _ => return Err(ParsingError::MethodHandle5or8NotPointingToMethodRef),
                    };
                }
            }
            _ => {}
        };

        Ok(())
    }
}
