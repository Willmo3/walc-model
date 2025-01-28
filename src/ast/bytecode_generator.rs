// Given an AST, convert it to a bytecode representation.
// In a transport-cost dominated environment (such as WebAssembly),
// we recommend translating to bytecode on the frontend. This translator then serves as a reference.
// Author: Will Morris

use crate::ast::token::Token;
use crate::interp::bytecode_interpreter::Opcode;
use crate::interp::bytecode_interpreter::Opcode::{ADD, DIVIDE, MULTIPLY, PUSH, SUBTRACT};

/// Given an ast, generate a list of bytes corresponding to walc bytecode.
pub fn generate(ast: &Token) -> Vec<u8> {
    let mut code = Vec::new();

    let mut generator_fn = | token: &Token | {
        match token {
            Token::Number { value } => {
                // Add push operation to bytecode and append floating point rep of number.
                code.push(Opcode::byte_from_opcode(&PUSH));
                code.extend_from_slice(&f64::to_le_bytes(*value));
            },
            Token::Add { .. } => code.push(Opcode::byte_from_opcode(&ADD)),
            Token::Subtract { .. } => code.push(Opcode::byte_from_opcode(&SUBTRACT)),
            Token::Multiply { .. } => code.push(Opcode::byte_from_opcode(&MULTIPLY)),
            Token::Divide { .. } => code.push(Opcode::byte_from_opcode(&DIVIDE)),
        }
    };

    ast.postorder_traverse(&mut generator_fn);
    code
}

#[cfg(test)]
mod tests {
    use crate::ast::bytecode_generator::generate;
    use crate::ast::token::Token;
    use crate::interp::bytecode_interpreter::interpret;

    #[test]
    fn test_add() {
        // 1 + -2
        let left = Box::new(Token::Number { value: 1.0 });
        let right = Box::new(Token::Number { value: -2.0 });
        let add = Token::Add { left, right };

        let bytecode = generate(&add);
        assert_eq!(-1.0, interpret(&bytecode).unwrap());
    }

    #[test]
    fn test_subtract() {
        // 1 - 2
        let left = Box::new(Token::Number { value: 1.0 });
        let right = Box::new(Token::Number { value: 2.0 });
        let subtract = Token::Subtract { left, right };

        let bytecode = generate(&subtract);
        assert_eq!(-1.0, interpret(&bytecode).unwrap());
    }

    #[test]
    fn test_multiply() {
        // 2 * -2
        let left = Box::new(Token::Number { value: 2.0 });
        let right = Box::new(Token::Number { value: -2.0 });
        let multiply = Token::Multiply { left, right };

        let bytecode = generate(&multiply);
        assert_eq!(-4.0, interpret(&bytecode).unwrap());
    }

    #[test]
    fn test_divide() {
        // -2 / 4
        let left = Box::new(Token::Number { value: -2.0 });
        let right = Box::new(Token::Number { value: 4.0 });
        let div = Token::Divide { left, right };

        let bytecode = generate(&div);
        assert_eq!(-0.5, interpret(&bytecode).unwrap());
    }

    #[test]
    fn test_divide_zero() {
        // 2 / 0
        let left = Box::new(Token::Number { value: 1.0 });
        let right = Box::new(Token::Number { value: 0.0 });
        let div = Token::Divide { left, right };

        let bytecode = generate(&div);
        assert_eq!(Err("Cannot divide by zero.\nNo result.\n".to_string()), interpret(&bytecode));
    }
}
