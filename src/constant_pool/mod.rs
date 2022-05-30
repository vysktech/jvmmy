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
