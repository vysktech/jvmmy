#[derive(Debug)]
pub enum ConstantPoolInfo {
    // tag=9
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    // tag=10
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    // tag=11
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    // tag=7
    Class {
        name_index: u16,
    },
    // tag=8
    String {
        string_index: u16,
    },
    // tag=3
    Integer {
        bytes: u32,
    },
    // tag=4
    Float {
        bytes: u32,
    },
    // tag=5
    Long {
        high_bytes: u32,
        low_bytes: u32,
    },
    // tag=6
    Double {
        high_bytes: u32,
        low_bytes: u32,
    },
    // tag=12
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    // tag=1
    Utf8 {
        // length: u16,
        // bytes: [u8; length],
        string: String,
    },
    // tag=15
    MethodHandle {
        length: u16,
        reference_kind: u8,
        reference_index: u8,
    },
    // tag=16
    MethodType {
        descriptor_index: u16,
    },
    // tag=17
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    // tag=18
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    // tag=19
    Module {
        name_index: u16,
    },
    // tag=20
    Package {
        name_index: u16,
    },
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
