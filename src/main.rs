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
use crate::constant_pool::{ConstantPoolInfo, ConstantPoolRuntime};
use crate::util::ByteReader;

use std::fs;
use std::fs::read;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::field::FieldFlags;

fn main() {
    let dir = "/home/vysk/dev/jvm/jvmmy";
    let filename = "OnePlusConst.class";
    let path = format!("{}/{}", dir, filename);
    let contents = read(path).expect("Failed to read file");

    let main_class: Class;
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

        main_class = Class {
            name: class_name.to_string(),
            super_class_name: super_class_name.to_string(),
            methods,
            constant_pool: class_file.constant_pool,
        };
    }

    // This is where we start interpreting code
    let mut classes = Vec::new();
    classes.push(main_class);
    let mut vm = VirtualMachine::new(classes);
    vm.start();
}

pub struct VirtualMachine {
    classes: Vec<Class>,
    objects: Vec<Object>,
    // frame_stack: Vec<Frame>,
}

impl VirtualMachine {
    pub fn new(classes: Vec<Class>) -> VirtualMachine {
        VirtualMachine {
            classes,
            objects: Vec::new(),
            // frame_stack: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        let mut frame_stack: Vec<Frame> = Vec::new();

        // Instance init methods: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.9.1
        // Class init methods: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.9.2

        match self.classes[0].methods.iter().position(|method|
            method.name == "main" && method.descriptor == "([Ljava/lang/String;)V")
        {
            // TODO: Might have to initiate first frame.local_vars with ref to `this`
            Some(method_index) => self.invoke_method(method_index, 0, &mut frame_stack),
            None => panic!("Expected main method")
        }
    }

    pub fn invoke_method(&self, method_index: usize, class_index: usize, frame_stack: &mut Vec<Frame>) {
        let class = &self.classes[class_index];
        let method = &class.methods[method_index];
        println!("Class#method: {}#{}", class.name, method.name);
        // self.frame_stack.push(Frame {
        frame_stack.push(Frame {
            local_vars: Vec::new(),
            local_var_types: Vec::new(),
            op_stack: Vec::new(),
            // constant_pool: class.constant_pool.borrow(),
        });
        for attribute in method.attributes.iter() {
            match attribute {
                // Code attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
                AttributeInfo::Code { code, .. } => {
                    let mut reader = ByteReader::new(code.to_vec());
                    while !reader.is_end() {
                        let instruction = reader.read_u8();

                        match instruction {
                            // aload_0
                            42 => {
                                println!("aload_0");
                                // let var = frame.local_vars[0];
                                // frame.op_stack.push(var);
                                eprintln!("Unimplemented opcode: aload_0");
                            }
                            // invokespecial
                            // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.10.1.9.invokespecial
                            183 => {
                                println!("invokespecial");
                                let arg1 = reader.read_u8();
                                let arg2 = reader.read_u8();
                                let index: u16 = (arg1 as u16) << 8 | (arg2 as u16) << 0;

                                let frame = frame_stack.last().unwrap();

                                // let resolved_method = &self.classes[class_index].methods[(index as usize) - 1];
                                // TODO: The called method should always receive 'this` as its first arg
                                self.invoke_method((index as usize) - 1, class_index, frame_stack);

                                // TODO: MIGHT BE THAT WE HAVE TO INDEX INTO `class.methods` instead!
                                //       That would make sense, with <init> going to `public static void main`

                                // match frame.constant_pool[(index as usize) - 1] {
                                //     ConstantPoolInfo::MethodRef { name_and_type_index, .. } => {
                                //         match frame.constant_pool[(name_and_type_index as usize) - 1] {
                                //             ConstantPoolInfo::NameAndType { name_index, descriptor_index } => {
                                //                 let method_name = match &frame.constant_pool[(name_index as usize) - 1] {
                                //                     ConstantPoolInfo::Utf8 { string } => string,
                                //                     _ => panic!("Expected Utf8")
                                //                 }.to_string();
                                //                 let method_descriptor = match &frame.constant_pool[(descriptor_index as usize) - 1] {
                                //                     ConstantPoolInfo::Utf8 { string } => string,
                                //                     _ => panic!("Expected Utf8")
                                //                 }.to_string();
                                //                 let resolved_method = class.methods.iter()
                                //                     .find(|&m| m.name == method_name && m.descriptor == method_descriptor)
                                //                     .expect("Expected to resolve method");
                                //                 if resolved_method.name == method.name {
                                //                     panic!("Resolved method is currently invoked method");
                                //                 }
                                //                 println!("Resolved method: {}", resolved_method.name);
                                //                 invoke_method(&resolved_method, class, self.frame_stack);
                                //             }
                                //             _ => panic!("Expected NameAndType")
                                //         }
                                //     }
                                //     _ => panic!("Expected MethodRef")
                                // }

                                eprintln!("Unimplemented opcode: invokespecial");
                            }
                            // iconst_0
                            3 => {
                                println!("iconst_0");
                                let frame = frame_stack.last_mut()
                                    .expect("Frame stack should contain at least 1 frame");
                                frame.op_stack.push(2);
                            }
                            // iconst_2
                            5 => {
                                println!("iconst_2");
                                let frame = frame_stack.last_mut()
                                    .expect("Frame stack should contain at least 1 frame");
                                frame.op_stack.push(2);
                            }
                            // aaload
                            50 => {
                                println!("aaload");
                                eprintln!("Unimplemented opcode: aaload");
                            }
                            // new
                            // https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.10.1.9.new
                            187 => {
                                println!("new");
                                let arg1 = reader.read_u8();
                                let arg2 = reader.read_u8();
                                let index: u16 = (arg1 as u16) << 8 | (arg2 as u16) << 0;
                                // &frame.constant_pool[index as usize];
                                eprintln!("Unimplemented opcode: new");
                            }
                            // dup
                            89 => {
                                println!("dup");
                                let frame = frame_stack.last_mut()
                                    .expect("Frame stack should contain at least 1 frame");
                                // TODO: Check whether top value is long/double, in which case we dup 2x
                                frame.op_stack.push(*frame.op_stack.last().unwrap());
                            }
                            // return
                            177 => {
                                frame_stack.pop();
                            }
                            _ => panic!("Unknown opcode: {}", instruction)
                        }
                    }
                }
                _ => continue
            }
        }

        // self.frame_stack.pop(); // TODO: Check spec: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.6
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

pub struct Object {
    class: Class,
    fields: HashMap<String, Field>,
}

pub struct Field {
    name: String,
    flags: u16,
    value: FieldValue,
}

pub enum FieldValue {
    Integer { value: u16 },
    String { value: String },
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
