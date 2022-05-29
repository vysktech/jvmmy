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
