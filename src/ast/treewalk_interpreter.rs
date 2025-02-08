use crate::ast::ast::ASTNode;
use crate::bytecode::bytecode_interpreter::Opcode::EXP;

/// Interpret a walc ast as a program.
/// Return the result of the computation, or the errors encountered.
pub fn interpret(ast: &ASTNode) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    let mut errors= String::new();

    let mut visit_fn = |token: &ASTNode| {
        match token {
            ASTNode::Add {..} | ASTNode::Subtract {..} | ASTNode::Multiply {..} | ASTNode::Divide {..}
                | ASTNode::Exponentiate {..} => {
                if stack.len() < 2 {
                    errors.push_str("Insufficient operands for binary operation!\n");
                    return
                }

                // Pop in reverse order
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                match token {
                    ASTNode::Add {..} => stack.push(left + right),
                    ASTNode::Subtract {..} => stack.push(left - right),
                    ASTNode::Multiply {..} => stack.push(left * right),
                    ASTNode::Divide {..} => {
                        if right == 0.0 {
                            errors.push_str("Cannot divide by zero.\n");
                        } else {
                            stack.push(left / right)
                        }
                    },
                    ASTNode::Exponentiate { .. } => stack.push(left.powf(right)),
                    _ => {
                        errors.push_str("Internal error: invalid token.\n");
                        return
                    }
                }
            }
            ASTNode::Number { value } => { stack.push(*value) }
        }
    };

    ast.postorder_traverse(&mut visit_fn);
    if stack.is_empty() {
        errors.push_str("No result.\n");
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(stack.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::ASTNode;
    use crate::ast::treewalk_interpreter::interpret;

    #[test]
    fn test_add() {
        // 1 + -2
        let left = Box::new(ASTNode::Number { value: 1.0 });
        let right = Box::new(ASTNode::Number { value: -2.0 });
        let add = ASTNode::Add { left, right };

        assert_eq!(-1.0, interpret(&add).unwrap());
    }

    #[test]
    fn test_subtract() {
        // 1 - 2
        let left = Box::new(ASTNode::Number { value: 1.0 });
        let right = Box::new(ASTNode::Number { value: 2.0 });
        let subtract = ASTNode::Subtract { left, right };

        assert_eq!(-1.0, interpret(&subtract).unwrap());
    }

    #[test]
    fn test_multiply() {
        // 2 * -2
        let left = Box::new(ASTNode::Number { value: 2.0 });
        let right = Box::new(ASTNode::Number { value: -2.0 });
        let multiply = ASTNode::Multiply { left, right };

        assert_eq!(-4.0, interpret(&multiply).unwrap());
    }

    #[test]
    fn test_divide() {
        // -2 / 4
        let left = Box::new(ASTNode::Number { value: -2.0 });
        let right = Box::new(ASTNode::Number { value: 4.0 });
        let div = ASTNode::Divide { left, right };

        assert_eq!(-0.5, interpret(&div).unwrap());
    }

    #[test]
    fn test_divide_zero() {
        // 2 / 0
        let left = Box::new(ASTNode::Number { value: 1.0 });
        let right = Box::new(ASTNode::Number { value: 0.0 });
        let div = ASTNode::Divide { left, right };

        assert_eq!(Err("Cannot divide by zero.\nNo result.\n".to_string()), interpret(&div));
    }

    #[test]
    fn test_double_exponentiate() {
        // 2 ** 3 ** 2
        let two = Box::new(ASTNode::Number { value: 2.0 });
        let three = Box::new(ASTNode::Number { value: 3.0 });
        let two_two = Box::new(ASTNode::Number { value: 2.0 });

        let three_to_two = Box::new(ASTNode::Exponentiate { left: three, right: two_two } );
        let final_ast = Box::new(ASTNode::Exponentiate { left: two, right: three_to_two });

        assert_eq!(Ok(512.0), interpret(&final_ast));
    }
}