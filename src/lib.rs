use crate::bytecode::bytecode_generator;
use crate::frontend::{lexer, parser};

/// Bytecode support for the walc programming language.
/// Author: Will Morris
mod bytecode {
    pub mod bytecode_interpreter;
    pub mod bytecode_generator;
    mod opcode;
    mod stackframe;
}

/// Walc AST operations, including treewalk interpreter.
/// Author: Will Morris
mod ast {
    pub mod ast;
}

/// Frontend for the Walc interpreter.
/// Author: Will Morris
mod frontend {
    pub mod lexer;
    pub mod parser;
}

/// Interface for Walc language. Takes a source program and returns the result of the computation,
/// Or an error.
pub fn interpret(source_code: &str) -> Result<String, String> {
    let tokens = match lexer::lex(source_code) {
        Ok(tokens) => tokens,
        Err(lex_error) => return Err(String::from(lex_error)),
    };
    let ast = match parser::parse(tokens) {
        Some(Ok(ast)) => ast,
        Some(Err(parse_error)) => return Err(String::from(parse_error)),
        None => return Err(String::from("")),
    };
    let bytecode = bytecode_generator::generate(&ast);
    match bytecode::bytecode_interpreter::execute(&bytecode) {
        Ok(value) => Ok(format!("{}", value)),
        Err(runtime_error) => Err(String::from(runtime_error))
    }
}

#[cfg(test)]
mod tests {
    use crate::interpret;

    #[test]
    fn test_interpret_div() {
        let source = "(3 + 5 + 2 - 2) * 2 / 8";
        assert_eq!(Ok("2".to_string()), interpret(source));
    }

    #[test]
    fn test_div_zero() {
        let source = "1 / 0";
        assert_eq!(Err("Cannot divide by zero.\nNo result.\n".to_string()), interpret(source));
    }

    #[test]
    fn test_double_exponentiate() {
        let source = "2**3**2";
        assert_eq!(Ok("512".to_string()), interpret(source));
    }

    #[test]
    fn test_assign() {
        let source = "x = 3 - 2";
        assert_eq!(Ok("1".to_string()), interpret(source));
    }
}