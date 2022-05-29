use crate::attr;
use attr::AttributeInfo;

pub struct MethodInfo {
    // TODO: Should be of type MethodFlags?
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

pub trait MethodFlags {
    fn is_public(&self) -> bool;
    fn is_private(&self) -> bool;
    fn is_protected(&self) -> bool;
    fn is_static(&self) -> bool;
    fn is_final(&self) -> bool;
    fn is_synchronized(&self) -> bool;
    fn is_bridge(&self) -> bool;
    fn is_varargs(&self) -> bool;
    fn is_native(&self) -> bool;
    fn is_abstract(&self) -> bool;
    fn is_strict(&self) -> bool;
    fn is_synthetic(&self) -> bool;
}

impl MethodFlags for u16 {
    fn is_public(&self) -> bool { self & 0x0001 == 0x0001 }
    fn is_private(&self) -> bool { self & 0x0002 == 0x0002 }
    fn is_protected(&self) -> bool { self & 0x0004 == 0x0004 }
    fn is_static(&self) -> bool { self & 0x0008 == 0x0008 }
    fn is_final(&self) -> bool { self & 0x0010 == 0x0010 }
    fn is_synchronized(&self) -> bool { self & 0x0020 == 0x0020 }
    fn is_bridge(&self) -> bool { self & 0x0040 == 0x0040 }
    fn is_varargs(&self) -> bool { self & 0x0080 == 0x0080 }
    fn is_native(&self) -> bool { self & 0x0100 == 0x0100 }
    fn is_abstract(&self) -> bool { self & 0x0400 == 0x0400 }
    fn is_strict(&self) -> bool { self & 0x0800 == 0x0800 }
    fn is_synthetic(&self) -> bool { self & 0x1000 == 0x1000 }
}
