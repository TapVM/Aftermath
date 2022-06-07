use super::errors::CpNodeError;
use super::ErrorAttributes;
use super::{Attributes, ClassFile, CpNode, ParsingError, StackMapFrame, VerificationTypeInfo};

pub struct Verifier<'a>
{
    class: ClassFile<'a>,
    bootstrap_methods: Vec<usize>,
}

impl<'a> Verifier<'a>
{
    pub fn new(class: ClassFile<'a>) -> Self
    {
        let bootstrap_methods: Vec<_> = class
            .attributes
            .iter()
            .enumerate()
            .filter_map(|x| {
                if let Attributes::BootstrapMethods(_) = x.1 {
                    Some(x.0)
                } else {
                    None
                }
            })
            .collect();

        Self {
            class,
            bootstrap_methods,
        }
    }

    pub fn verify(self) -> Result<ClassFile<'a>, ParsingError<'a>>
    {
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

        let this_class = if let CpNode::Utf8(z) =
            if let CpNode::Class(z) = &class.cp[class.this_class.to_u2() as usize - 1] {
                &class.cp[z.name_index.to_u2() as usize - 1]
            } else {
                unreachable!()
            } {
            z.bytes
        } else {
            unreachable!()
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

    fn verify_class_attributes(&self) -> Result<(), ParsingError<'a>>
    {
        let class_attributes = &self.class.attributes;
        let cp = &self.class.cp;

        for z in class_attributes {
            match z {
                Attributes::SourceFile(z) => {
                    if !matches!(&cp[z.sourcefile_index.to_u2() as usize], CpNode::Utf8(..)) {
                        return Err(ParsingError::InvalidIndexFromAttributeToNode(
                            ErrorAttributes::SourceFile,
                            CpNodeError::Utf8,
                            "sourcefile_index",
                        ));
                    }
                }
                Attributes::InnerClass(z) => {}
                Attributes::EnclosingMethod(z) => {}
                Attributes::SourceDebugExt(z) => {}
                Attributes::BootstrapMethods(z) => {}
                Attributes::Module(z) => {}
                Attributes::ModulePackages(z) => {}
                Attributes::ModuleMainClass(z) => {}
                Attributes::NestHost(z) => {}
                Attributes::NestMembers(z) => {}
                Attributes::Record(z) => {}
                Attributes::PermittedSubclasses(z) => {}
                Attributes::Synthetic(z) => {}
                Attributes::Deprecated(z) => {}
                Attributes::Signature(z) => {}
                Attributes::RuntimeVisibleAnnotations(z) => {}
                Attributes::RuntimeInvisibleAnnotations(z) => {}
                Attributes::RuntimeVisibleTypeAnnotations(z) => {}
                Attributes::RuntimeInvisibleTypeAnnotations(z) => {}
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn verify_attributes(&self) -> Result<(), ParsingError<'a>>
    {
        for z in &self.class.attributes {
            if !matches!(
                z,
                Attributes::SourceFile(..)
                    | Attributes::InnerClass(..)
                    | Attributes::EnclosingMethod(..)
                    | Attributes::SourceDebugExt(..)
                    | Attributes::BootstrapMethods(..)
                    | Attributes::Module(..)
                    | Attributes::ModulePackages(..)
                    | Attributes::ModuleMainClass(..)
                    | Attributes::NestHost(..)
                    | Attributes::NestMembers(..)
                    | Attributes::Record(..)
                    | Attributes::PermittedSubclasses(..)
                    | Attributes::Synthetic(..)
                    | Attributes::Deprecated(..)
                    | Attributes::Signature(..)
                    | Attributes::RuntimeVisibleAnnotations(..)
                    | Attributes::RuntimeInvisibleAnnotations(..)
                    | Attributes::RuntimeVisibleTypeAnnotations(..)
                    | Attributes::RuntimeInvisibleTypeAnnotations(..)
            ) {
                return Err(ParsingError::InvalidClassAttributes);
            }

            if let Attributes::Record(z) = z {
                for z in &z.components {
                    for z in &z.attributes {
                        if !matches!(
                            z,
                            Attributes::Signature(..)
                                | Attributes::RuntimeVisibleAnnotations(..)
                                | Attributes::RuntimeInvisibleAnnotations(..)
                                | Attributes::RuntimeVisibleTypeAnnotations(..)
                                | Attributes::RuntimeInvisibleTypeAnnotations(..)
                        ) {
                            return Err(ParsingError::InvalidRecordComponentInfoAttributes);
                        }
                    }
                }
            }
        }

        for z in &self.class.fields {
            for z in &z.attributes {
                if !matches!(
                    z,
                    Attributes::Value(..)
                        | Attributes::Synthetic(..)
                        | Attributes::Deprecated(..)
                        | Attributes::Signature(..)
                        | Attributes::RuntimeVisibleAnnotations(..)
                        | Attributes::RuntimeInvisibleAnnotations(..)
                        | Attributes::RuntimeVisibleTypeAnnotations(..)
                        | Attributes::RuntimeInvisibleTypeAnnotations(..)
                ) {
                    return Err(ParsingError::InvalidFieldInfoAttributes);
                }
            }
        }

        for z in &self.class.methods {
            for z in &z.attributes {
                if !matches!(
                    z,
                    Attributes::Code(..)
                        | Attributes::Exceptions(..)
                        | Attributes::RuntimeVisibleAnnotations(..)
                        | Attributes::RuntimeVisibleTypeAnnotations(..)
                        | Attributes::RuntimeInvisibleAnnotations(..)
                        | Attributes::RuntimeInvisibleTypeAnnotations(..)
                        | Attributes::RuntimeVisibleParameterAnnotations(..)
                        | Attributes::RuntimeInvisibleParameterAnnotations(..)
                        | Attributes::AnnotationDefault(..)
                        | Attributes::MethodParameters(..)
                        | Attributes::Synthetic(..)
                        | Attributes::Deprecated(..)
                        | Attributes::Signature(..)
                ) {
                    return Err(ParsingError::InvalidMethodInfoAttributes);
                }

                if let Attributes::Code(z) = z {
                    for z in &z.attributes {
                        if !matches!(
                            z,
                            Attributes::LineNumberTable(..)
                                | Attributes::LocalVariableTable(..)
                                | Attributes::LocalVariableTypeTable(..)
                                | Attributes::StackMapTable(..)
                                | Attributes::RuntimeVisibleTypeAnnotations(..)
                                | Attributes::RuntimeInvisibleTypeAnnotations(..)
                        ) {
                            return Err(ParsingError::InvalidCodeAttributes);
                        }
                    }
                }
            }
        }

        // -----------------------------------------------------------------------------------------

        self.verify_class_attributes()?;
        Ok(())
    }

    fn verify_attributes_internal(&self, attribute: Attributes) -> Result<(), ParsingError<'a>>
    {
        let cp = &self.class.cp;
        Ok(())

        // match attribute {
        //     Attributes::Value(cv) => {
        //         if !matches!(
        //             cp[cv.value_index.to_u2() as usize],
        //             CpNode::Integer(_)
        //                 | CpNode::Float(_)
        //                 | CpNode::Long(_)
        //                 | CpNode::Double(_)
        //                 | CpNode::String(_)
        //         ) {
        //             return Err(ParsingError::InvalidIndexFromAttributeToNodes(
        //                 ErrorAttributes::Value,
        //                 &[
        //                     CpNodeError::Integer,
        //                     CpNodeError::Float,
        //                     CpNodeError::Long,
        //                     CpNodeError::Double,
        //                     CpNodeError::String,
        //                 ],
        //                 "value_index",
        //             ));
        //         }
        //     }
        //     Attributes::Code(code) => {
        //         let code_arr = code.code;

        //         if code_arr.len() >= 65536 || code_arr.is_empty() {
        //             return Err(ParsingError::CodeAttributeCodeLength);
        //         }

        //         for y in &code.exception_table {
        //             if code_arr.get(y.start_pc.to_u2() as usize).is_none()
        //                 || code_arr.get(y.end_pc.to_u2() as usize).is_none()
        //                 || code_arr.get(y.handler_pc.to_u2() as usize).is_none()
        //             {
        //                 return Err(ParsingError::InvalidIndexesInCodeAttribute);
        //             }

        //             if y.catch_type.to_u2() != 0
        //                 && !matches!(cp[y.catch_type.to_u2() as usize], CpNode::Class(..))
        //             {
        //                 return Err(ParsingError::InvalidIndexFromAttributeToNode(
        //                     ErrorAttributes::Code,
        //                     CpNodeError::Class,
        //                     "catch_type",
        //                 ));
        //             }
        //         }

        //         for attr in code.attributes {
        //             self.verify_attributes(attr)?;
        //         }
        //     }
        //     Attributes::StackMapTable(smt) => {
        //         for smf in smt.entries {
        //             match smf {
        //                 StackMapFrame::SameLocals1StackItemFrame(z) => {
        //                     if let VerificationTypeInfo::ObjectVariableInfo(y) = z.stack {
        //                         if !matches!(cp[y.cp_index.to_u2() as usize], CpNode::Class(..)) {
        //                             return Err(ParsingError::InvalidIndexFromAttributeToNode(
        //                                 ErrorAttributes::StackMapTable,
        //                                 CpNodeError::Class,
        //                                 "cp_index",
        //                             ));
        //                         }
        //                     }
        //                 }
        //                 StackMapFrame::SameLocals1StackItemFrameExtended(z) => {
        //                     if let VerificationTypeInfo::ObjectVariableInfo(y) = z.stack {
        //                         if !matches!(cp[y.cp_index.to_u2() as usize], CpNode::Class(..)) {
        //                             return Err(ParsingError::InvalidIndexFromAttributeToNode(
        //                                 ErrorAttributes::StackMapTable,
        //                                 CpNodeError::Class,
        //                                 "cp_index",
        //                             ));
        //                         }
        //                     }
        //                 }
        //                 StackMapFrame::AppendFrame(z) => {
        //                     for i in z.locals {
        //                         if let VerificationTypeInfo::ObjectVariableInfo(y) = i {
        //                             if !matches!(cp[y.cp_index.to_u2() as usize], CpNode::Class(..))
        //                             {
        //                                 return Err(ParsingError::InvalidIndexFromAttributeToNode(
        //                                     ErrorAttributes::StackMapTable,
        //                                     CpNodeError::Class,
        //                                     "cp_index",
        //                                 ));
        //                             }
        //                         }
        //                     }
        //                 }
        //                 StackMapFrame::FullFrame(z) => {
        //                     for i in z.locals {
        //                         if let VerificationTypeInfo::ObjectVariableInfo(y) = i {
        //                             if !matches!(cp[y.cp_index.to_u2() as usize], CpNode::Class(..))
        //                             {
        //                                 return Err(ParsingError::InvalidIndexFromAttributeToNode(
        //                                     ErrorAttributes::StackMapTable,
        //                                     CpNodeError::Class,
        //                                     "cp_index",
        //                                 ));
        //                             }
        //                         }
        //                     }
        //                     for i in z.stack {
        //                         if let VerificationTypeInfo::ObjectVariableInfo(y) = i {
        //                             if !matches!(cp[y.cp_index.to_u2() as usize], CpNode::Class(..))
        //                             {
        //                                 return Err(ParsingError::InvalidIndexFromAttributeToNode(
        //                                     ErrorAttributes::StackMapTable,
        //                                     CpNodeError::Class,
        //                                     "cp_index",
        //                                 ));
        //                             }
        //                         }
        //                     }
        //                 }
        //                 _ => {}
        //             }
        //         }
        //     }
        //     Attributes::Exceptions(exception) => {
        //         for i in exception.exception_index_table {
        //             if !matches!(cp[i.to_u2() as usize], CpNode::Class(..)) {
        //                 return Err(ParsingError::InvalidIndexFromAttributeToNode(
        //                     ErrorAttributes::Exceptions,
        //                     CpNodeError::Class,
        //                     "exception_index_table",
        //                 ));
        //             }
        //         }
        //     }
        //     Attributes::InnerClass(innerclass) => {
        //         for k in innerclass.classes {
        //             if let CpNode::Class(y) = &cp[k.inner_class_info_index.to_u2() as usize] {
        //                 #[allow(non_snake_case)]
        //                 let C = y;
        //             } else {
        //                 return Err(ParsingError::InvalidIndexFromAttributeToNode(
        //                     ErrorAttributes::InnerClass,
        //                     CpNodeError::Class,
        //                     "inner_class_info_index",
        //                 ));
        //             }
        //         }
        //     }
        //     Attributes::EnclosingMethod(enclosingmethod) => {}
        //     Attributes::Synthetic(_) => todo!(),
        //     Attributes::Signature(_) => todo!(),
        //     Attributes::SourceFile(_) => todo!(),
        //     Attributes::SourceDebugExt(_) => todo!(),
        //     Attributes::LineNumberTable(_) => todo!(),
        //     Attributes::LocalVariableTable(_) => todo!(),
        //     Attributes::LocalVariableTypeTable(_) => todo!(),
        //     Attributes::Deprecated(_) => todo!(),
        //     Attributes::RuntimeVisibleAnnotations(_) => todo!(),
        //     Attributes::RuntimeInvisibleAnnotations(_) => todo!(),
        //     Attributes::RuntimeVisibleParameterAnnotations(_) => todo!(),
        //     Attributes::RuntimeInvisibleParameterAnnotations(_) => todo!(),
        //     Attributes::RuntimeVisibleTypeAnnotations(_) => todo!(),
        //     Attributes::RuntimeInvisibleTypeAnnotations(_) => todo!(),
        //     Attributes::AnnotationDefault(_) => todo!(),
        //     Attributes::BootstrapMethods(_) => todo!(),
        //     Attributes::MethodParameters(_) => todo!(),
        //     Attributes::Module(_) => todo!(),
        //     Attributes::ModulePackages(_) => todo!(),
        //     Attributes::ModuleMainClass(_) => todo!(),
        //     Attributes::NestHost(_) => todo!(),
        //     Attributes::NestMembers(_) => todo!(),
        //     Attributes::Record(_) => todo!(),
        //     Attributes::PermittedSubclasses(_) => todo!(),
        // }

        // Ok(())
    }

    fn verify_cp_node(&self, node: &CpNode) -> Result<(), ParsingError<'a>>
    {
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
                } else if if let Attributes::BootstrapMethods(z) =
                    &self.class.attributes[bootstrap_methods[0]]
                {
                    z
                } else {
                    unreachable!()
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
                } else if if let Attributes::BootstrapMethods(z) =
                    &self.class.attributes[bootstrap_methods[0]]
                {
                    z
                } else {
                    unreachable!()
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
