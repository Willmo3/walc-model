/// interpreters for the walc programming language.
/// Author: Will Morris
pub mod interp {
    mod bytecode_interpreter;
}

/// Walc syntax tree traversal and manipulation.
/// Author: Will Morris
pub mod ast {
    pub mod bytecode_generator;
    pub mod token;
}
