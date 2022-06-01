#[derive(Debug)]
pub enum AttributeInfo {
    ConstantValue {
        constantvalue_index: u16, // Points at constant_pool
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_tables: Vec<ExceptionTable>,
        attributes: Vec<AttributeInfo>,
    },
    LineNumberTable {
        entries: Vec<LineNumberTableEntry>,
    },
    SourceFile {
        sourcefile_index: u16, // Points at constant_pool
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
