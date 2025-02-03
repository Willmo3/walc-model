use crate::bytecode::bytecode_generator;
use crate::frontend::{lexer, parser};

/// Bytecode support for the walc programming language.
/// Author: Will Morris
mod bytecode {
    pub mod bytecode_interpreter;
    pub mod bytecode_generator;
}

/// Walc AST operations, including treewalk interpreter.
/// Author: Will Morris
mod ast {
    pub mod ast;
    pub mod treewalk_interpreter;
}

/// Frontend for the Walc interpreter.
/// Author: Will Morris
mod frontend {
    pub mod lexer;
    pub mod parser;
}

/// Interface for Walc language. Takes a source program and returns the result of the computation,
/// Or an error.
pub fn interpret(source_code: &str) -> Result<f64, String> {
    let tokens = lexer::lex(source_code);
    // Note: why does the parser consume the tokens? Look into changing.
    let ast = parser::parse(tokens).unwrap();
    let bytecode = bytecode_generator::generate(&ast);
    bytecode::bytecode_interpreter::interpret(&bytecode)
}

#[cfg(test)]
mod tests {
    use crate::interpret;

    #[test]
    fn test_interpret_div() {
        let source = "(3 + 5 + 2 - 2) * 2 / 8";
        assert_eq!(Ok(2.0), interpret(source));
    }

    #[test]
    fn test_div_zero() {
        let source = "1 / 0";
        assert_eq!(Err("Cannot divide by zero.\nNo result.\n".to_string()), interpret(source));
    }
}