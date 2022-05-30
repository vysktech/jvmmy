use crate::attr::{AttributeInfo, AttributeInfoDetails, ExceptionTable, LineNumberTableEntry};
use crate::attr::AttributeInfoDetails::SourceFile;
use crate::class::ClassFile;
use crate::class::ClassFlags;
use crate::constant_pool::ConstantPoolInfo;
use crate::field::FieldInfo;
use crate::method::MethodInfo;
use crate::util::ByteReader;

pub struct ClassFileLoader {
    reader: ByteReader,
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<ConstantPoolInfo>,
    // TODO: Should be of ClassFlags type?
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    // These values are pointers into the constant pool
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
}

impl ClassFileLoader {
    pub fn new(class_file_contents: Vec<u8>) -> ClassFileLoader {
        ClassFileLoader {
            reader: ByteReader::new(class_file_contents),
            minor_version: 0,
            major_version: 0,
            constant_pool: Vec::new(),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn load(mut self) -> ClassFile {
        let magic: u32 = self.reader.read_u32();
        assert_eq!(magic, 0xcafebabe);
        self.minor_version = self.reader.read_u16();
        print!("minor_version: {}\n", self.minor_version);
        self.major_version = self.reader.read_u16();
        print!("major_version: {}\n", self.major_version);
        let constant_pool_count: u16 = self.reader.read_u16();
        print!("constant_pool_count: {}\n", constant_pool_count);

        for _ in 0..constant_pool_count - 1 { // Constant pool index starts at 1
            let tag = self.reader.read_u8();
            match tag {
                9 => {
                    let info = ConstantPoolInfo::FieldRef {
                        class_index: self.reader.read_u16(),
                        name_and_type_index: self.reader.read_u16(),
                    };
                    println!("{:?}", info);
                    self.constant_pool.push(info);
                }
                10 => {
                    let info = ConstantPoolInfo::MethodRef {
                        class_index: self.reader.read_u16(),
                        name_and_type_index: self.reader.read_u16(),
                    };
                    println!("{:?}", info);
                    self.constant_pool.push(info);
                }
                8 => {
                    let info = ConstantPoolInfo::String {
                        string_index: self.reader.read_u16(),
                    };
                    println!("{:?}", info);
                    self.constant_pool.push(info);
                }
                7 => {
                    let info = ConstantPoolInfo::Class {
                        name_index: self.reader.read_u16(),
                    };
                    println!("{:?}", info);
                    self.constant_pool.push(info);
                }
                1 => {
                    let mut length: usize = self.reader.read_u16().into();
                    let string = self.reader.read_str(length);
                    let info = ConstantPoolInfo::Utf8 {
                        string: string.to_string(),
                    };
                    println!("{:?}", info);
                    self.constant_pool.push(info);
                }
                12 => {
                    let info = ConstantPoolInfo::NameAndType {
                        name_index: self.reader.read_u16(),
                        descriptor_index: self.reader.read_u16(),
                    };
                    println!("{:?}", info);
                    self.constant_pool.push(info);
                }
                _ => panic!("unknown constant pool tag: {}\n", tag)
            }
        }

        // TODO: Should be of type ClassFlags?
        self.access_flags = self.reader.read_u16();
        println!("is_public {}", ClassFlags::is_public(&self.access_flags));
        println!("is_final {}", ClassFlags::is_final(&self.access_flags));
        println!("is_super_special {}", ClassFlags::is_super_special(&self.access_flags));
        println!("is_interface {}", ClassFlags::is_interface(&self.access_flags));
        println!("is_abstract {}", ClassFlags::is_abstract(&self.access_flags));
        println!("is_synthetic {}", ClassFlags::is_synthetic(&self.access_flags));
        println!("is_annotation {}", ClassFlags::is_annotation(&self.access_flags));
        println!("is_enum {}", ClassFlags::is_synthetic(&self.access_flags));
        println!("is_module {}", ClassFlags::is_enum(&self.access_flags));

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

        self.this_class = self.reader.read_u16();

        let this_class_info = &self.constant_pool[(self.this_class as usize) - 1]; // Constant pool index starts at 1
        match this_class_info {
            ConstantPoolInfo::Class { .. } => println!("Yup, it's a class, alright"),
            _ => panic!("Value of this_class should point to a class in the constant pool")
        }

        // Spec: either zero, or, if nonzero, a valid index into the constant pool
        self.super_class = self.reader.read_u16();

        let interfaces_count: u16 = self.reader.read_u16();
        for _ in 0..interfaces_count {
            self.interfaces.push(self.reader.read_u16());
        }

        let fields_count: u16 = self.reader.read_u16();
        for _ in 0..fields_count {
            let access_flags = self.reader.read_u16();
            let name_index = self.reader.read_u16(); // TODO: Check constant pool for corresponding utf8
            let descriptor_index = self.reader.read_u16();
            let attributes_count = self.reader.read_u16();
            let attributes: Vec<AttributeInfo> = self.read_attributes(attributes_count);
            self.fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            });
        }

        let methods_count = self.reader.read_u16();
        for _ in 0..methods_count {
            let access_flags = self.reader.read_u16();
            let name_index = self.reader.read_u16(); // TODO: Check constant pool for corresponding utf8
            let descriptor_index = self.reader.read_u16();
            // TODO: For descriptor_index, check:
            // - If method is in class, and name is <init>, then descriptor must denote a void method
            // - If name of method is <clinit>, then descriptor must denote a void method, and in class
            //   file >=51.0, method should have zero args
            let attributes_count = self.reader.read_u16();
            let attributes: Vec<AttributeInfo> = self.read_attributes(attributes_count);
            self.methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            });
        }

        let attributes_count = self.reader.read_u16();
        self.attributes = self.read_attributes(attributes_count);

        println!("Class file loaded; read {}/{} bytes",
                 self.reader.index, self.reader.contents.len());

        ClassFile {
            minor_version: self.minor_version,
            major_version: self.major_version,
            constant_pool: self.constant_pool,
            access_flags: self.access_flags,
            this_class: self.this_class,
            super_class: self.super_class,
            interfaces: self.interfaces,
            fields: self.fields,
            methods: self.methods,
            attributes: self.attributes,
        }
    }

    fn read_attributes(&mut self, attributes_count: u16) -> Vec<AttributeInfo> {
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            let attribute_name_index = self.reader.read_u16();
            let attribute_name = match &self.constant_pool[(attribute_name_index as usize) - 1] {
                ConstantPoolInfo::Utf8 { string } => string,
                _ => panic!("Expected to find attribute name")
            };
            println!("Attribute name: {}", attribute_name);

            self.reader.read_u32(); // attribute_length
            let mut info: Vec<AttributeInfoDetails> = Vec::new();

            let attribute = match attribute_name.as_str() {
                "Code" => {
                    let max_stack = self.reader.read_u16();
                    let max_locals = self.reader.read_u16();

                    let code_length = self.reader.read_u32();
                    let code: Vec<u8> = self.reader.read_vec_u8(code_length as usize);

                    let exception_table_length = self.reader.read_u16();
                    let mut exception_tables: Vec<ExceptionTable> = Vec::new();
                    for _ in 0..exception_table_length {
                        exception_tables.push(ExceptionTable {
                            start_pc: self.reader.read_u16(),
                            end_pc: self.reader.read_u16(),
                            handler_pc: self.reader.read_u16(),
                            catch_type: self.reader.read_u16(),
                        });
                    }

                    let code_attributes_count = self.reader.read_u16();
                    let code_attributes = self.read_attributes(code_attributes_count);

                    AttributeInfoDetails::Code {
                        max_stack,
                        max_locals,
                        code,
                        exception_tables,
                        attributes: code_attributes,
                    }
                }
                "LineNumberTable" => {
                    let line_number_table_length = self.reader.read_u16();
                    let mut entries: Vec<LineNumberTableEntry> = Vec::new();
                    for _ in 0..line_number_table_length {
                        entries.push(LineNumberTableEntry {
                            start_pc: self.reader.read_u16(),
                            line_number: self.reader.read_u16(),
                        })
                    }
                    AttributeInfoDetails::LineNumberTable {
                        entries
                    }
                }
                "SourceFile" => {
                    SourceFile {
                        sourcefile_index: self.reader.read_u16(),
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
}
