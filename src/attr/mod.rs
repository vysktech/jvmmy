pub struct AttributeInfo {
    pub(crate) attribute_name_index: u16,
    // Length does not include first 6 bytes representing attribute_name_index & attribute_length
    // attribute_length: u32,
    // info: [AttributeInfoDetails],
    pub(crate) info: Vec<AttributeInfoDetails>
}

pub enum AttributeInfoDetails {
    ConstantValue {
        // Points at constant_pool
        constantvalue_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTable>,
        attribute_info: Vec<AttributeInfo>
        // code_length: u32,
        // code: [u8],
        // exception_table_length: u16,
        // exception_table: [ExceptionTable],
        // attributes_count: u16,
        // attribute_info: [AttributeInfo]
    }
}

pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_c: u16,
    catch_type: u16,
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
