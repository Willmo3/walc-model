// Given an AST, convert it to a bytecode representation.
// In a transport-cost dominated environment (such as WebAssembly),
// we recommend translating to bytecode on the frontend. This translator then serves as a reference.
// Author: Will Morris

use crate::ast::ast::ASTNode;
use crate::bytecode::opcode::Opcode;
use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, MULTIPLY, PUSH, SUBTRACT};


/// Given an ast, generate a list of bytes corresponding to walc bytecode.
pub fn generate(ast: &ASTNode) -> Vec<u8> {
    let mut code = Vec::new();

    let mut generator_fn = | token: &ASTNode| {
        match token {
            ASTNode::Number { value } => {
                // Add push operation to bytecode and append floating point rep of number.
                code.push(Opcode::byte_from_opcode(&PUSH));
                code.extend_from_slice(&f64::to_le_bytes(*value));
            },
            ASTNode::Add { .. } => code.push(Opcode::byte_from_opcode(&ADD)),
            ASTNode::Subtract { .. } => code.push(Opcode::byte_from_opcode(&SUBTRACT)),
            ASTNode::Multiply { .. } => code.push(Opcode::byte_from_opcode(&MULTIPLY)),
            ASTNode::Divide { .. } => code.push(Opcode::byte_from_opcode(&DIVIDE)),
            ASTNode::Exponentiate { .. } => code.push(Opcode::byte_from_opcode(&EXP)),
        }
    };

    ast.postorder_traverse(&mut generator_fn);
    code
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::ASTNode;
    use crate::bytecode::bytecode_generator::generate;
    use crate::bytecode::bytecode_interpreter::execute;

    #[test]
    fn test_add() {
        // 1 + -2
        let left = Box::new(ASTNode::Number { value: 1.0 });
        let right = Box::new(ASTNode::Number { value: -2.0 });
        let add = ASTNode::Add { left, right };

        let bytecode = generate(&add);
        assert_eq!(-1.0, execute(&bytecode).unwrap());
    }

    #[test]
    fn test_subtract() {
        // 1 - 2
        let left = Box::new(ASTNode::Number { value: 1.0 });
        let right = Box::new(ASTNode::Number { value: 2.0 });
        let subtract = ASTNode::Subtract { left, right };

        let bytecode = generate(&subtract);
        assert_eq!(-1.0, execute(&bytecode).unwrap());
    }

    #[test]
    fn test_multiply() {
        // 2 * -2
        let left = Box::new(ASTNode::Number { value: 2.0 });
        let right = Box::new(ASTNode::Number { value: -2.0 });
        let multiply = ASTNode::Multiply { left, right };

        let bytecode = generate(&multiply);
        assert_eq!(-4.0, execute(&bytecode).unwrap());
    }

    #[test]
    fn test_divide() {
        // -2 / 4
        let left = Box::new(ASTNode::Number { value: -2.0 });
        let right = Box::new(ASTNode::Number { value: 4.0 });
        let div = ASTNode::Divide { left, right };

        let bytecode = generate(&div);
        assert_eq!(-0.5, execute(&bytecode).unwrap());
    }

    #[test]
    fn test_divide_zero() {
        // 2 / 0
        let left = Box::new(ASTNode::Number { value: 1.0 });
        let right = Box::new(ASTNode::Number { value: 0.0 });
        let div = ASTNode::Divide { left, right };

        let bytecode = generate(&div);
        assert_eq!(Err("Cannot divide by zero.\nNo result.\n".to_string()), execute(&bytecode));
    }
}
