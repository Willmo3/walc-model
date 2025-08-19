// Given an AST, convert it to a bytecode representation.
// In a transport-cost dominated environment (such as WebAssembly),
// we recommend translating to bytecode on the frontend. This translator then serves as a reference.
// Author: Will Morris

use crate::ast::ast::ASTNode;
use crate::bytecode::opcode::Opcode;
use crate::bytecode::opcode::Opcode::{ADD, ASSIGN, DIVIDE, EXP, MULTIPLY, PUSH, READVAR, SUBTRACT};


/// Given an ast, generate a list of bytes corresponding to walc bytecode.
pub fn generate(ast: &ASTNode) -> Vec<u8> {
    let mut code = Vec::new();

    let mut generator_fn = | token: &ASTNode| {
        match token {
            // TODO: change assignment to writevar
            ASTNode::Assignment { name, .. } => {
                code.push(Opcode::byte_from_opcode(&ASSIGN));
                // Invariant: no name will have more than 256 characters.
                code.push(name.len() as u8);
                code.extend_from_slice(&name.as_bytes());
                // Linear code preceding assignment will have produced value onto stack.
            }
            ASTNode::VarRead { name } => {
                code.push(Opcode::byte_from_opcode(&READVAR));
                // Invariant: no name will have more than 256 characters.
                code.push(name.len() as u8);
                code.extend_from_slice(&name.as_bytes());
                // Value of variable will evaluate to whatever is in dynamic scope.
            }
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
    use crate::bytecode::opcode::Opcode;
    use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, PUSH, READVAR};

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

    #[test]
    fn test_assign() {
        // x_var = 3 ** -1 - 1

        let three = Box::new(ASTNode::Number { value: 3.0 });
        let minus1 = Box::new(ASTNode::Number { value: -1.0 });
        let exp = Box::new(ASTNode::Exponentiate { left: three, right: minus1 });
        let one = Box::new(ASTNode::Number { value: 1.0 });
        let subtract = Box::new(ASTNode::Subtract { left: exp, right: one });
        let root = ASTNode::Assignment { name: String::from("x_var"), value: subtract };

        let bytecode = generate(&root);
        assert_eq!(-0.6666666666666667, execute(&bytecode).unwrap());
    }

    #[test]
    fn test_readvar() {
        // Test whether bytecode for a readvar expression can be generated, separately of its execution.

        // 3 / x_var
        let name = "x_var";

        let three = Box::new(ASTNode::Number { value: 3.0 });
        let xvar = Box::new(ASTNode::VarRead { name: name.to_string() });
        let div = ASTNode::Divide { left: three, right: xvar };

        let mut expected_bytecode: Vec<u8> = Vec::new();
        expected_bytecode.push(Opcode::byte_from_opcode(&PUSH));
        expected_bytecode.extend_from_slice(&f64::to_le_bytes(3.0));
        expected_bytecode.push(Opcode::byte_from_opcode(&READVAR));
        expected_bytecode.push(name.len() as u8);
        expected_bytecode.extend_from_slice(&name.as_bytes());
        expected_bytecode.push(Opcode::byte_from_opcode(&DIVIDE));

        assert_eq!(expected_bytecode, generate(&div));
    }
}
