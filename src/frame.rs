use crate::constant_pool::ConstantPoolInfo;

pub struct Frame {
    // Size of local_vars is compile-time, determined by Code attribute.
    // Each byte holds a single primitive type, reference or returnAddress.
    // Longs and doubles occupy two bytes.
    // They are referenced by the first index, but the whole value occupies index and index+1.
    // For class instance methods, index 0 (zero) is always 'this'.
    // For class methods, index 0 and onwards can be used for parameters.
    pub local_vars: Vec<u8>,
    // The index of local_vars is important, so we keep the VariableType in a separate list.
    // We could make a struct "Variable" to hold both the type and its value, but that would mess
    // with the indexes of local_vars (because longs and doubles should take up two indexes).
    pub local_var_types: Vec<VariableType>,
    // Size of op_stack is compile-time, determined by Code attribute.
    // The operand stack is last-in-first-out (LIFO).
    // It contains values and constants from fields and local variables.
    // It also contains parameters and return values for methods.
    // Each value takes up one unit (of size), but longs and doubles take up two units.
    pub op_stack: Vec<u8>,
    // pub constant_pool: &[ConstantPoolInfo],
}

pub enum VariableType {
    Reference
}
