use std::ops::RangeInclusive;
pub const MAGIC: u32 = 0xCAFEBABE;
pub const MAJOR_VERSION_RANGE: RangeInclusive<u16> = 45..=61;

pub mod class_acc_flags
{
    pub const PUBLIC: u16 = 0x0001;
    pub const FINAL: u16 = 0x0010;
    pub const SUPER: u16 = 0x0020;
    pub const INTERFACE: u16 = 0x0200;
    pub const ABSTRACT: u16 = 0x0400;
    pub const SYNTHETIC: u16 = 0x1000;
    pub const ANNOTATION: u16 = 0x2000;
    pub const ENUM: u16 = 0x4000;
    pub const MODULE: u16 = 0x8000;
}
