extern crate core;

mod attr;
mod class;
mod method;
mod constant_pool;
mod field;
mod util;
mod frame;

use crate::class::ClassFile;
use crate::class::loading::ClassFileLoader;
use crate::attr::AttributeInfo;
use crate::frame::Frame;
use crate::constant_pool::ConstantPoolInfo;
use crate::util::ByteReader;

use std::fs;
use std::fs::read;
use std::borrow::Borrow;

fn main() {
    let dir = "/home/vysk/dev/jvm/jvmmy";
    let filename = "OnePlusOne.class";
    let path = format!("{}/{}", dir, filename);
    let contents = read(path).expect("Failed to read file");

    let class: Class;
    {
        let mut loader = ClassFileLoader::new(contents);
        let mut class_file = loader.load();

        // This is where we start resolving the class file into a run-time class representation

        let class_name = match class_file.constant_pool[(class_file.this_class as usize) - 1] {
            ConstantPoolInfo::Class { name_index } => {
                class_file.find_name(name_index)
            }
            _ => panic!("Expected Class attribute at constant_pool[{}]", class_file.this_class.to_owned())
        };

        let super_class_name = match class_file.constant_pool[(class_file.super_class as usize) - 1] {
            ConstantPoolInfo::Class { name_index } => {
                class_file.find_name(name_index)
            }
            _ => panic!("Expected Class attribute at constant_pool[{}]", class_file.super_class.to_owned())
        };

        let mut methods: Vec<Method> = Vec::new();

        while let Some(method_info) = class_file.methods.pop() {
            // let attributes = method_info.attributes.iter().filter(|&attr| match attr {
            //     AttributeInfo::Code { .. } => true,
            //     _ => false
            // });

            methods.push(Method {
                name: class_file.find_name(method_info.name_index).to_string(),
                descriptor: class_file.find_name(method_info.descriptor_index).to_string(),
                class_name: class_name.to_string(),
                attributes: method_info.attributes,
            });
        }

        class = Class {
            name: class_name.to_string(),
            super_class_name: super_class_name.to_string(),
            methods,
            constant_pool: class_file.constant_pool,
        };
    }

    // This is where we start interpreting code

    let mut frame_stack: Vec<Frame> = Vec::new();

    for method in class.methods {
        frame_stack.push(Frame {
            local_vars: Vec::new(),
            local_var_types: Vec::new(),
            op_stack: Vec::new(),
            constant_pool: class.constant_pool.borrow(),
        });
        for attribute in method.attributes {
            match attribute {
                AttributeInfo::Code { code, .. } => {
                    let mut reader = ByteReader::new(code.to_vec());
                    let instruction = reader.read_u8();

                    match instruction {
                        // aload_0
                        42 => {
                            println!("aload_0");
                            // let var = frame.local_vars[0];
                            // frame.op_stack.push(var);
                        }
                        // invokespecial
                        183 => {
                            println!("invokespecial");
                            let arg1 = reader.read_u8();
                            let arg2 = reader.read_u8();
                            let index: u16 = (arg1 as u16) << 8 | (arg2 as u16) << 0;
                            // &frame.constant_pool[index as usize];
                        }
                        // iconst_2
                        5 => {
                            println!("iconst_2");
                            let frame = frame_stack.last_mut()
                                .expect("Frame stack should contain at least 1 frame");
                            frame.op_stack.push(2);
                        }
                        _ => {}
                    }
                }
                _ => continue
            }
        }

        // frame_stack.pop(); // TODO: Check spec: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.6
    }
}

pub struct Class {
    // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.2.1
    // Either a binary name or an interface name
    name: String,
    // Refers to the binary (a.k.a. internal) name of the class
    super_class_name: String,
    methods: Vec<Method>,
    // TODO: Should be run-time constant pool, i.e. not the constant pool *info*
    constant_pool: Vec<ConstantPoolInfo>,
}

pub struct Method {
    // The name of the method (simply as it appears in source code)
    name: String,
    // The method descriptor for the method:
    // Object m(int i, double d, Thread t) {...}
    // is:
    // (IDLjava/lang/Thread;)Ljava/lang/Object;
    descriptor: String,
    // Refers to the binary (a.k.a. internal) name of the class
    class_name: String,
    // Should be Code attributes, mostly
    attributes: Vec<AttributeInfo>,
}
