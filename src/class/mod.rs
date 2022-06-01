pub mod loading;

use crate::constant_pool;
use crate::field;
use crate::attr;
use crate::method;

use constant_pool::ConstantPoolInfo;
use field::FieldInfo;
use attr::AttributeInfo;
use method::MethodInfo;

pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantPoolInfo>,
    // TODO: Should be of ClassFlags type?
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn find_name(&self, index: u16) -> String {
        match &self.constant_pool[(index as usize) - 1] {
            ConstantPoolInfo::Utf8 { string } => string.to_string(),
            _ => panic!("Expected to find a utf8 string at constant_pool[{}]", index)
        }
    }
}

pub trait ClassFlags {
    fn is_public(&self) -> bool;
    fn is_final(&self) -> bool;
    // Spec(ACC_SUPER): Treat superclass methods specially when invoked by the invokespecial instruction
    fn is_super_special(&self) -> bool;
    fn is_interface(&self) -> bool;
    fn is_abstract(&self) -> bool;
    fn is_synthetic(&self) -> bool;
    fn is_annotation(&self) -> bool;
    fn is_enum(&self) -> bool;
    fn is_module(&self) -> bool;
}

impl ClassFlags for u16 {
    fn is_public(&self) -> bool { self & 0x0001 == 0x0001 }
    fn is_final(&self) -> bool { self & 0x0010 == 0x0010 }
    fn is_super_special(&self) -> bool { self & 0x0020 == 0x0020 }
    fn is_interface(&self) -> bool { self & 0x0200 == 0x0200 }
    fn is_abstract(&self) -> bool { self & 0x0400 == 0x0400 }
    fn is_synthetic(&self) -> bool { self & 0x1000 == 0x1000 }
    fn is_annotation(&self) -> bool { self & 0x2000 == 0x2000 }
    fn is_enum(&self) -> bool { self & 0x4000 == 0x4000 }
    fn is_module(&self) -> bool { self & 0x8000 == 0x8000 }
}

impl ClassFlags for ClassFile {
    fn is_public(&self) -> bool { ClassFlags::is_public(&self.access_flags) }
    fn is_final(&self) -> bool { ClassFlags::is_final(&self.access_flags) }
    fn is_super_special(&self) -> bool { ClassFlags::is_super_special(&self.access_flags) }
    fn is_interface(&self) -> bool { ClassFlags::is_interface(&self.access_flags) }
    fn is_abstract(&self) -> bool { ClassFlags::is_abstract(&self.access_flags) }
    fn is_synthetic(&self) -> bool { ClassFlags::is_synthetic(&self.access_flags) }
    fn is_annotation(&self) -> bool { ClassFlags::is_annotation(&self.access_flags) }
    fn is_enum(&self) -> bool { ClassFlags::is_enum(&self.access_flags) }
    fn is_module(&self) -> bool { ClassFlags::is_module(&self.access_flags) }
}
