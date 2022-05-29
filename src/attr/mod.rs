#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    // Length does not include first 6 bytes representing attribute_name_index & attribute_length
    // attribute_length: u32,
    // info: [AttributeInfoDetails],
    pub info: Vec<AttributeInfoDetails>,
}

#[derive(Debug)]
pub enum AttributeInfoDetails {
    ConstantValue {
        // Points at constant_pool
        constantvalue_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_tables: Vec<ExceptionTable>,
        attributes: Vec<AttributeInfo>,
        // code_length: u32,
        // code: [u8],
        // exception_table_length: u16,
        // exception_table: [ExceptionTable],
        // attributes_count: u16,
        // attribute_info: [AttributeInfo]
    },
    LineNumberTable {
        entries: Vec<LineNumberTableEntry>,
    },
    SourceFile {
        sourcefile_index: u16,
    },
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

// Required:
// - ConstantValue
// - Code
// - StackMapTable
// - BootstrapMethods
// - NestHost
// - NestMembers
// - PermittedSubclasses

// Optional (required for correct interpretation or just useful):
// - Exceptions
// - InnerClasses
// - EnclosingMethod
// - Synthetic
// - Signature
// - Record
// - SourceFile
// - LineNumberTable
// - LocalVariableTable
// - LocalVariableTypeTable

// Optional (metadata):
// - SourceDebugExtension
// - Deprecated
// - RuntimeVisibleAnnotations
// - RuntimeInvisibleAnnotations
// - RuntimeVisibleParameterAnnotations
// - RuntimeInvisibleParameterAnnotations
// - RuntimeVisibleTypeAnnotations
// - RuntimeInvisibleTypeAnnotations
// - AnnotationDefault
// - MethodParameters
// - Module
// - ModulePackages
// - ModuleMainClass
