use super::{ClassFile, ParsingError, U2};

pub fn verify(class: ClassFile) -> Result<ClassFile, ParsingError> {
    // Version Verification

    if !(45..=61).contains(&class.major_v.to_u2()) {
        return Err(ParsingError::InvalidMajorV);
    }

    Ok(class)
}
