use super::errors::CpNodeError;
use super::{Attributes, ClassFile, CpNode, ParsingError};

pub struct Verifier<'a> {
    class: ClassFile<'a>,
    bootstrap_methods: Vec<usize>,
}

impl<'a> Verifier<'a> {
    pub fn new(class: ClassFile<'a>) -> Self {
        let bootstrap_methods = class
            .attributes
            .iter()
            .enumerate()
            .filter_map(|x| match x.1 {
                Attributes::BootstrapMethods(_) => Some(x.0),
                _ => None,
            })
            .collect::<Vec<_>>();

        Self {
            class,
            bootstrap_methods,
        }
    }

    pub fn verify(self) -> Result<ClassFile<'a>, ParsingError<'a>> {
        let class = &self.class;

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

            let mut filtered = self.class.attributes.iter().filter(|z| {
                matches!(
                    z,
                    Attributes::Module(..)
                        | Attributes::ModulePackages(..)
                        | Attributes::ModuleMainClass(..)
                        | Attributes::InnerClass(..)
                        | Attributes::SourceFile(..)
                        | Attributes::SourceDebugExt(..)
                        | Attributes::RuntimeVisibleAnnotations(..)
                        | Attributes::RuntimeInvisibleAnnotations(..)
                )
            });

            let has_module = filtered.any(|x| matches!(x, Attributes::Module(..)));

            if !(filtered.next().is_none() && has_module) {
                return Err(ParsingError::InvalidAttributesAsModule);
            }
        }

        Ok(self.class)
    }

    fn verify_cp_node(&self, node: &CpNode) -> Result<(), ParsingError<'a>> {
        let cp = &self.class.cp;
        let bootstrap_methods = &self.bootstrap_methods;

        match node {
            CpNode::Class(class) => {
                let node = &cp[class.name_index.to_u2() as usize - 1];
                if let CpNode::Utf8(string) = node {
                    string.verify_binary_class_or_interface_name()?;
                    self.verify_cp_node(node)?;
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::Class,
                        CpNodeError::Utf8,
                        "name_index",
                    ));
                };
            }

            CpNode::FieldRef(fieldref) => {
                let class_index = fieldref.class_index;
                let name_and_type = fieldref.name_and_type_index;

                let node = &cp[class_index.to_u2() as usize - 1];
                if let CpNode::Class(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::FieldRef,
                        CpNodeError::Class,
                        "class_index",
                    ));
                };

                let node = &cp[name_and_type.to_u2() as usize - 1];
                if let CpNode::NameAndType(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::FieldRef,
                        CpNodeError::NameAndType,
                        "name_and_type",
                    ));
                }
            }

            CpNode::MethodRef(methodref) => {
                let class_index = methodref.class_index;
                let name_and_type = methodref.name_and_type_index;

                let node = &cp[class_index.to_u2() as usize - 1];
                if let CpNode::Class(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::MethodRef,
                        CpNodeError::Class,
                        "class_index",
                    ));
                };

                let node = &cp[name_and_type.to_u2() as usize - 1];
                if let CpNode::NameAndType(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::MethodRef,
                        CpNodeError::NameAndType,
                        "name_and_type",
                    ));
                }
            }

            CpNode::InterfaceMethodRef(interfacemethodref) => {
                let class_index = interfacemethodref.class_index;
                let name_and_type = interfacemethodref.name_and_type_index;

                let node = &cp[class_index.to_u2() as usize - 1];
                if let CpNode::Class(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::InterfaceMethodRef,
                        CpNodeError::Class,
                        "class_index",
                    ));
                };

                let node = &cp[name_and_type.to_u2() as usize - 1];
                if let CpNode::NameAndType(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::InterfaceMethodRef,
                        CpNodeError::NameAndType,
                        "name_and_type",
                    ));
                }
            }

            CpNode::String(string) => {
                let node = &cp[string.string_index.to_u2() as usize - 1];
                if let CpNode::Utf8(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::String,
                        CpNodeError::Utf8,
                        "string_index",
                    ));
                };
            }

            CpNode::MethodType(methodtype) => {
                let node = &cp[methodtype.descriptor_index.to_u2() as usize - 1];
                if let CpNode::Utf8(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::MethodType,
                        CpNodeError::Utf8,
                        "descriptor_index",
                    ));
                };
            }

            CpNode::Module(module) => {
                let node = &cp[module.name_index.to_u2() as usize - 1];
                if let CpNode::Utf8(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::Module,
                        CpNodeError::Utf8,
                        "name_index",
                    ));
                };
            }

            CpNode::Package(package) => {
                let package = &cp[package.name_index.to_u2() as usize - 1];
                if let CpNode::Utf8(_) = package {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::Package,
                        CpNodeError::Utf8,
                        "name_index",
                    ));
                };
            }

            CpNode::Dynamic(dynamic) => {
                let bootstrap_method_attr_index = dynamic.bootstrap_method_attr_index.to_u2();

                if bootstrap_methods.len() != 1 {
                    return Err(ParsingError::InvalidAmountOfBootStrapMethodsInClass);
                } else if match &self.class.attributes[bootstrap_methods[0]] {
                    Attributes::BootstrapMethods(z) => z,
                    _ => unreachable!(),
                }
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
                if let CpNode::NameAndType(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::Dynamic,
                        CpNodeError::NameAndType,
                        "name_and_type_index",
                    ));
                };
            }

            CpNode::NameAndType(nameandtype) => {
                let name_index = nameandtype.name_index.to_u2();

                let node = &cp[name_index as usize - 1];
                if let CpNode::Utf8(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::NameAndType,
                        CpNodeError::Utf8,
                        "name_index",
                    ));
                };
            }

            CpNode::InvokeDynamic(dynamic) => {
                let bootstrap_method_attr_index = dynamic.bootstrap_method_attr_index.to_u2();

                if bootstrap_methods.len() != 1 {
                    return Err(ParsingError::InvalidAmountOfBootStrapMethodsInClass);
                } else if match &self.class.attributes[bootstrap_methods[0]] {
                    Attributes::BootstrapMethods(z) => z,
                    _ => unreachable!(),
                }
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
                if let CpNode::NameAndType(_) = node {
                    self.verify_cp_node(node)?
                } else {
                    return Err(ParsingError::InvalidIndexFromNodeToNode(
                        CpNodeError::InvokeDynamic,
                        CpNodeError::NameAndType,
                        "name_and_type_index",
                    ));
                };
            }

            CpNode::MethodHandle(methodhandle) => {
                let reference_kind = &methodhandle.reference_kind;
                let reference_index = methodhandle.reference_index.to_u2();

                if !(1..=9).contains(reference_kind) {
                    return Err(ParsingError::InvalidReferenceKind);
                } else if (1..=4).contains(reference_kind) {
                    let node = &cp[reference_index as usize - 1];
                    if let CpNode::FieldRef(_) = node {
                        self.verify_cp_node(node)?
                    } else {
                        return Err(ParsingError::MethodHandle1to4NotPointingToFieldRef);
                    };
                } else if reference_kind == &5 || reference_kind == &8 {
                    let node = &cp[reference_index as usize - 1];
                    if let CpNode::MethodRef(_) = node {
                        self.verify_cp_node(node)?
                    } else {
                        return Err(ParsingError::MethodHandle5or8NotPointingToMethodRef);
                    };
                }
            }

            _ => {}
        };

        Ok(())
    }
}
