/// Bytecode support for the walc programming language.
/// Author: Will Morris
pub mod bytecode {
    pub mod bytecode_interpreter;
    pub mod bytecode_generator;
}

/// Walc AST operations, including treewalk interpreter.
/// Author: Will Morris
pub mod ast {
    pub mod ast;
    pub mod treewalk_interpreter;
}

pub mod frontend {
    pub mod lexer;
    pub mod parser;
}