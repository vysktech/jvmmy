mod attr;
mod constant_pool;
mod field;

use constant_pool::ConstantPoolInfo;
use field::{FieldFlags, FieldInfo};

use std::fs;
use std::fs::read;
use std::io::Read;
use std::process::exit;
use crate::attr::{AttributeInfo, AttributeInfoDetails};

struct Reader {
    index: usize,
    contents: Vec<u8>,
}

impl Reader {
    pub const fn new(contents: Vec<u8>) -> Self {
        Reader { index: 0, contents }
    }

    fn read_u8(&mut self) -> u8 {
        let result: u8 = self.contents[self.index];
        self.index += 1;
        result
    }

    fn read_u16(&mut self) -> u16 {
        let result: u16 = (self.contents[self.index] as u16) << 8
            | (self.contents[self.index + 1] as u16) << 0;
        self.index += 2;
        result
    }

    fn read_u32(&mut self) -> u32 {
        let result: u32 = (self.contents[self.index] as u32) << 24
            | (self.contents[self.index + 1] as u32) << 16
            | (self.contents[self.index + 2] as u32) << 8
            | (self.contents[self.index + 3] as u32) << 0;
        self.index += 4;
        result
    }

    fn read_str(&mut self, length: usize) -> &str {
        let range = self.index..self.index + length;
        let result: &str = std::str::from_utf8(&self.contents[range])
            .expect("Invalid UTF-8 value");
        self.index += length;
        result
    }
}

fn main() {
    println!("Hello, world!");

    let dir = "/home/vysk/dev/jvm/jvmmy";
    let filename = "HelloWorld.class";
    let path = format!("{}/{}", dir, filename);

    let contents = fs::read(path)
        .expect("Failed to read file");
    let mut reader = Reader::new(contents);

    let magic: u32 = reader.read_u32();
    assert_eq!(magic, 0xcafebabe);
    let minor_version: u16 = reader.read_u16();
    print!("minor_version: {}\n", minor_version);
    let major_version: u16 = reader.read_u16();
    print!("major_version: {}\n", major_version);
    let constant_pool_count: u16 = reader.read_u16();
    print!("constant_pool_count: {}\n", constant_pool_count);

    let mut constant_pool: Vec<ConstantPoolInfo> = Vec::new();

    for _ in 0..constant_pool_count - 1 { // Constant pool index starts at 1
        let tag = reader.read_u8();
        match tag {
            9 => {
                let info = ConstantPoolInfo::FieldRef {
                    class_index: reader.read_u16(),
                    name_and_type_index: reader.read_u16(),
                };
                println!("{:?}", info);
                constant_pool.push(info);
            }
            10 => {
                let info = ConstantPoolInfo::MethodRef {
                    class_index: reader.read_u16(),
                    name_and_type_index: reader.read_u16(),
                };
                println!("{:?}", info);
                constant_pool.push(info);
            }
            8 => {
                let info = ConstantPoolInfo::String {
                    string_index: reader.read_u16(),
                };
                println!("{:?}", info);
                constant_pool.push(info);
            }
            7 => {
                let info = ConstantPoolInfo::Class {
                    name_index: reader.read_u16(),
                };
                println!("{:?}", info);
                constant_pool.push(info);
            }
            1 => {
                let mut length: usize = reader.read_u16().into();
                let string = reader.read_str(length);
                let info = ConstantPoolInfo::Utf8 {
                    string: string.to_string(),
                };
                println!("{:?}", info);
                constant_pool.push(info);
            }
            12 => {
                let info = ConstantPoolInfo::NameAndType {
                    name_index: reader.read_u16(),
                    descriptor_index: reader.read_u16(),
                };
                println!("{:?}", info);
                constant_pool.push(info);
            }
            _ => panic!("unknown constant pool tag: {}\n", tag)
        }
    }

    // TODO: Should be of type ClassFlags?
    let access_flags: u16 = reader.read_u16();
    println!("is_public {}", access_flags.is_public());
    println!("is_final {}", access_flags.is_private());
    println!("is_super {}", access_flags.is_protected());
    println!("is_interface {}", access_flags.is_static());
    println!("is_abstract {}", access_flags.is_final());
    println!("is_synthetic {}", access_flags.is_volatile());
    println!("is_annotation {}", access_flags.is_transient());
    println!("is_enum {}", access_flags.is_synthetic());
    println!("is_module {}", access_flags.is_enum());

    let this_class: usize = reader.read_u16().into();

    let this_class_info = &constant_pool[this_class - 1]; // Constant pool index starts at 1
    match this_class_info {
        ConstantPoolInfo::Class { .. } => println!("Yup, it's a class, alright"),
        _ => panic!("Value of this_class should point to a class in the constant pool")
    }

    // Spec: either zero, or, if nonzero, a valid index into the constant pool
    let super_class = reader.read_u16();

    let interfaces_count: u16 = reader.read_u16();
    let mut interfaces: Vec<usize> = Vec::new(); // These values are pointers into the constant pool
    for _ in 0..interfaces_count {
        interfaces.push(reader.read_u16().into());
    }

    let fields_count: u16 = reader.read_u16();
    for _ in 0..fields_count {
        let access_flags = reader.read_u16();
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        let attributes_count = reader.read_u16();
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            let attribute_name_index = reader.read_u16();
            let info: Vec<AttributeInfoDetails> = Vec::new();
            for _ in 0..attribute_name_index {
                let attribute_name = match &constant_pool[(attribute_name_index as usize) - 1] {
                    ConstantPoolInfo::Utf8 { string } => string,
                    _ => panic!("Expected to find attribute name")
                };
                println!("attr name: {}", attribute_name);
            }
            attributes.push(AttributeInfo {
                attribute_name_index,
                info
            });
        }
    }

    let methods_count = reader.read_u16();
    for _ in 0..methods_count {
        //
    }
}
