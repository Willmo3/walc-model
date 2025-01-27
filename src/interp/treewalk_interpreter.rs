use crate::ast::token::Token;

/// Interpret a walc ast as a program.
/// Return the result of the computation, or the errors encountered.
pub fn interpret(ast: &Token) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    let mut errors= String::new();

    let mut visit_fn = |token: &Token| {
        match token {
            Token::Add {..} | Token::Subtract {..} | Token::Multiply {..} | Token::Divide {..} => {
                if stack.len() < 2 {
                    errors.push_str("Insufficient operands for binary operation!\n");
                    return
                }

                // Pop in reverse order
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                match token {
                    Token::Add {..} => stack.push(left + right),
                    Token::Subtract {..} => stack.push(left - right),
                    Token::Multiply {..} => stack.push(left * right),
                    Token::Divide {..} => {
                        if right == 0.0 {
                            errors.push_str("Cannot divide by zero.\n");
                        } else {
                            stack.push(left / right)
                        }
                    },
                    _ => {
                        errors.push_str("Internal error: invalid token.\n");
                        return
                    }
                }
            }
            Token::Number { value } => { stack.push(*value) }
        }
    };

    ast.postorder_traverse(&mut visit_fn);
    if stack.is_empty() {
        errors.push_str("No results.\n");
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(stack.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::token::Token;
    use crate::interp::treewalk_interpreter::interpret;

    #[test]
    fn test_add() {
        // 1 + -2
        let left = Box::new(Token::Number { value: 1.0 });
        let right = Box::new(Token::Number { value: -2.0 });
        let add = Token::Add { left, right };

        assert_eq!(-1.0, interpret(&add).unwrap());
    }

    #[test]
    fn test_subtract() {
        // 1 - 2
        let left = Box::new(Token::Number { value: 1.0 });
        let right = Box::new(Token::Number { value: 2.0 });
        let subtract = Token::Subtract { left, right };

        assert_eq!(-1.0, interpret(&subtract).unwrap());
    }

    #[test]
    fn test_multiply() {
        // 2 * -2
        let left = Box::new(Token::Number { value: 2.0 });
        let right = Box::new(Token::Number { value: -2.0 });
        let multiply = Token::Multiply { left, right };

        assert_eq!(-4.0, interpret(&multiply).unwrap());
    }

    #[test]
    fn test_divide() {
        // -2 / 4
        let left = Box::new(Token::Number { value: -2.0 });
        let right = Box::new(Token::Number { value: 4.0 });
        let div = Token::Divide { left, right };

        assert_eq!(-0.5, interpret(&div).unwrap());
    }

    #[test]
    fn test_divide_zero() {
        // 2 / 0
        let left = Box::new(Token::Number { value: 1.0 });
        let right = Box::new(Token::Number { value: 0.0 });
        let div = Token::Divide { left, right };

        assert_eq!(Err("Cannot divide by zero.\nNo results.\n".to_string()), interpret(&div));
    }
}