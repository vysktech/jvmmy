use crate::attr;
use attr::AttributeInfo;

pub struct FieldInfo {
    // TODO: Should be of type FieldFlags?
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    // attributes_count: u16,
    pub attributes: Vec<AttributeInfo>
}

pub trait FieldFlags {
    fn is_public(&self) -> bool;
    fn is_private(&self) -> bool;
    fn is_protected(&self) -> bool;
    fn is_static(&self) -> bool;
    fn is_final(&self) -> bool;
    fn is_volatile(&self) -> bool;
    fn is_transient(&self) -> bool;
    fn is_synthetic(&self) -> bool;
    fn is_enum(&self) -> bool;
}

impl FieldFlags for u16 {
    fn is_public(&self) -> bool { self & 0x0001 == 0x0001 }
    fn is_private(&self) -> bool { self & 0x0002 == 0x0002 }
    fn is_protected(&self) -> bool { self & 0x0004 == 0x0004 }
    fn is_static(&self) -> bool { self & 0x0008 == 0x0008 }
    fn is_final(&self) -> bool { self & 0x0010 == 0x0010 }
    fn is_volatile(&self) -> bool { self & 0x0040 == 0x0040 }
    fn is_transient(&self) -> bool { self & 0x0080 == 0x0080 }
    fn is_synthetic(&self) -> bool { self & 0x1000 == 0x1000 }
    fn is_enum(&self) -> bool { self & 0x4000 == 0x4000 }
}
