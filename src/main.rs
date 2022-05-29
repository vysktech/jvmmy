mod attr;
mod class;
mod method;
mod constant_pool;
mod field;

use attr::{AttributeInfo, AttributeInfoDetails};
use class::ClassFile;
use method::MethodInfo;
use constant_pool::{ConstantPoolInfo, ClassFlags};
use field::{FieldFlags, FieldInfo};

use std::fs;
use std::fs::read;
use std::io::Read;
use std::process::exit;
use crate::attr::{ExceptionTable, LineNumberTableEntry};
use crate::AttributeInfoDetails::SourceFile;

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

    fn read_vec_u8(&mut self, length: usize) -> Vec<u8> {
        let range = self.index..self.index + length;
        let result: Vec<u8> = Vec::from(&self.contents[range]);
        self.index += length;
        result
    }
}

fn main() {
    println!("Hello, world!");

    let dir = "/home/vysk/dev/jvm/jvmmy";
    let filename = "HelloWorld.class";
    let path = format!("{}/{}", dir, filename);

    let contents = read(path)
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
    println!("is_public {}", ClassFlags::is_public(&access_flags));
    println!("is_final {}", ClassFlags::is_final(&access_flags));
    println!("is_super_special {}", ClassFlags::is_super_special(&access_flags));
    println!("is_interface {}", ClassFlags::is_interface(&access_flags));
    println!("is_abstract {}", ClassFlags::is_abstract(&access_flags));
    println!("is_synthetic {}", ClassFlags::is_synthetic(&access_flags));
    println!("is_annotation {}", ClassFlags::is_annotation(&access_flags));
    println!("is_enum {}", ClassFlags::is_synthetic(&access_flags));
    println!("is_module {}", ClassFlags::is_enum(&access_flags));

    // TODO: Spec says:
    // if access_flags.is_module() {
    //     return ClassFile {
    //         minor_version,
    //         major_version,
    //         this_class: "module-info",
    //         super_class: 0,
    //         interfaces: Vec::new(),
    //         fields: Vec::new(),
    //         methods: Vec::new()
    //     }
    // }

    let this_class: u16 = reader.read_u16();

    let this_class_info = &constant_pool[(this_class as usize) - 1]; // Constant pool index starts at 1
    match this_class_info {
        ConstantPoolInfo::Class { .. } => println!("Yup, it's a class, alright"),
        _ => panic!("Value of this_class should point to a class in the constant pool")
    }

    // Spec: either zero, or, if nonzero, a valid index into the constant pool
    let super_class = reader.read_u16();

    let interfaces_count: u16 = reader.read_u16();
    let mut interfaces: Vec<u16> = Vec::new(); // These values are pointers into the constant pool
    for _ in 0..interfaces_count {
        interfaces.push(reader.read_u16());
    }

    let mut fields: Vec<FieldInfo> = Vec::new();
    let fields_count: u16 = reader.read_u16();
    for _ in 0..fields_count {
        let access_flags = reader.read_u16();
        let name_index = reader.read_u16(); // TODO: Check constant pool for corresponding utf8
        let descriptor_index = reader.read_u16();
        let attributes_count = reader.read_u16();
        let attributes: Vec<AttributeInfo> = read_attributes(&mut reader, attributes_count, &constant_pool);
        fields.push(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        });
    }

    let mut methods: Vec<MethodInfo> = Vec::new();
    let methods_count = reader.read_u16();
    for _ in 0..methods_count {
        let access_flags = reader.read_u16();
        let name_index = reader.read_u16(); // TODO: Check constant pool for corresponding utf8
        let descriptor_index = reader.read_u16();
        // TODO: For descriptor_index, check:
        // - If method is in class, and name is <init>, then descriptor must denote a void method
        // - If name of method is <clinit>, then descriptor must denote a void method, and in class
        //   file >=51.0, method should have zero args
        let attributes_count = reader.read_u16();
        let attributes: Vec<AttributeInfo> = read_attributes(&mut reader, attributes_count, &constant_pool);
        methods.push(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        });
    }

    let attributes_count = reader.read_u16();
    let attributes = read_attributes(&mut reader, attributes_count, &constant_pool);

    println!("Reader finished; index: {}, content length: {}", reader.index, reader.contents.len());

    ClassFile {
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes,
    };
}

fn read_attributes(
    reader: &mut Reader,
    attributes_count: u16,
    constant_pool: &Vec<ConstantPoolInfo>,
) -> Vec<AttributeInfo> {
    let mut attributes: Vec<AttributeInfo> = Vec::new();
    for _ in 0..attributes_count {
        let attribute_name_index = reader.read_u16();
        let attribute_name = match &constant_pool[(attribute_name_index as usize) - 1] {
            ConstantPoolInfo::Utf8 { string } => string,
            _ => panic!("Expected to find attribute name")
        };
        println!("Attribute name: {}", attribute_name);

        reader.read_u32(); // attribute_length
        let mut info: Vec<AttributeInfoDetails> = Vec::new();

        let attribute = match attribute_name.as_str() {
            "Code" => {
                let max_stack = reader.read_u16();
                let max_locals = reader.read_u16();

                let code_length = reader.read_u32();
                let code: Vec<u8> = reader.read_vec_u8(code_length as usize);

                let exception_table_length = reader.read_u16();
                let mut exception_tables: Vec<ExceptionTable> = Vec::new();
                for _ in 0..exception_table_length {
                    exception_tables.push(ExceptionTable {
                        start_pc: reader.read_u16(),
                        end_pc: reader.read_u16(),
                        handler_pc: reader.read_u16(),
                        catch_type: reader.read_u16(),
                    });
                }

                let code_attributes_count = reader.read_u16();
                let code_attributes = read_attributes(reader, code_attributes_count, constant_pool);

                AttributeInfoDetails::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_tables,
                    attributes: code_attributes,
                }
            }
            "LineNumberTable" => {
                let line_number_table_length = reader.read_u16();
                let mut entries: Vec<LineNumberTableEntry> = Vec::new();
                for _ in 0..line_number_table_length {
                    entries.push(LineNumberTableEntry {
                        start_pc: reader.read_u16(),
                        line_number: reader.read_u16(),
                    })
                }
                AttributeInfoDetails::LineNumberTable {
                    entries
                }
            }
            "SourceFile" => {
                SourceFile {
                    sourcefile_index: reader.read_u16(),
                }
            }
            _ => panic!("Unknown attribute {}", attribute_name)
        };

        // println!("{:?}", attribute);

        info.push(attribute);

        attributes.push(AttributeInfo {
            attribute_name_index,
            info,
        });
    }
    attributes
}
