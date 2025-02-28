use crate::bytecode::opcode::Opcode;
use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, MULTIPLY, PUSH, SUBTRACT};
use crate::bytecode::stackframe::StackFrame;

const IMM_LEN: usize = 8;

/// Interpret a collection of bytes as a walc program.
/// Return f64 result of computation.
pub fn execute(bytes: &Vec<u8>) -> Result<f64, String> {
    let mut stack = StackFrame::new();
    interpret_frame(bytes, &mut stack)
}

/*
 * Interpret a single stack frame, recursing if a new level of depth found.
 * Returns result of interpreting frame.
 */
fn interpret_frame(bytes: &Vec<u8>, frame: &mut StackFrame) -> Result<f64, String> {
    let mut errors = String::new();
    let mut index = 0;

    while index < bytes.len() {
        let operation = bytes[index];
        match Opcode::opcode_from_byte(operation) {
            PUSH => {
                index += 1; // Skip opcode.

                let mut immediate_bytes = [0u8; IMM_LEN];
                immediate_bytes[..IMM_LEN].copy_from_slice(&bytes[index..(index + IMM_LEN)]);
                frame.push_value(f64::from_le_bytes(immediate_bytes));
                index += 8; // Read 8-bytes from bytecode value.
            },
            ADD | SUBTRACT | MULTIPLY | DIVIDE | EXP => {
                index += 1; // Skip opcode

                if frame.stack_size() < 2 {
                    errors.push_str("Binary operation attempted with insufficient operands!\n");
                    continue
                }

                // Operands pushed onto stack in reverse order.
                let right = frame.pop_value().unwrap();
                let left = frame.pop_value().unwrap();

                match Opcode::opcode_from_byte(operation) {
                    ADD => frame.push_value(left + right),
                    SUBTRACT => frame.push_value(left - right),
                    MULTIPLY => frame.push_value(left * right),
                    DIVIDE => {
                        if right == 0.0 {
                            errors.push_str("Cannot divide by zero.\n");
                            continue
                        }
                        frame.push_value(left / right)
                    },
                    EXP => frame.push_value(left.powf(right)),
                    _ => errors.push_str(&format!("Unknown binary operation: {}\n", operation)),
                }
            }
        }
    }

    if frame.stack_size() == 0 {
        errors.push_str("No result.\n");
    }

    if errors.is_empty() {
        Ok(frame.pop_value().unwrap())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::bytecode_interpreter::execute;

    #[test]
    fn test_add() {
        // 1 + 2
        let mut code = Vec::new();

        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(1.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(f64::MAX - 1.0));
        code.push(1u8);

        // Run calculation
        assert_eq!(execute(&code).unwrap(), f64::MAX);
    }

    #[test]
    fn test_subtract() {
        // 1 - 2
        let mut code = Vec::new();

        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(1.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(2u8);

        assert_eq!(execute(&code).unwrap(), -1.0);
    }

    #[test]
    fn test_multiply() {
        // 2 * 128
        let mut code = Vec::new();

        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(128.0));
        code.push(3u8);

        assert_eq!(execute(&code).unwrap(), 256.0);
    }

    #[test]
    fn test_divide() {
        // 2 / 4
        let mut code = Vec::new();

        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(4.0));
        code.push(4u8);

        assert_eq!(execute(&code).unwrap(), 0.5);
    }

    #[test]
    fn test_divide_zero() {
        // 2 / 0
        let mut code = Vec::new();

        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(0.0));
        code.push(4u8);

        assert_eq!(execute(&code), Err("Cannot divide by zero.\nNo result.\n".to_string()));
    }

    #[test]
    fn test_pow() {
        // 2 ** 2
        let mut code = Vec::new();
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(5u8);

        assert_eq!(execute(&code).unwrap(), 4.0);
    }

    #[test]
    fn test_double_exp() {
        // 2 ** 3 ** 2
        let mut code = Vec::new();

        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(3.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(5u8); // 3 ** 2
        code.push(5u8);

        assert_eq!(execute(&code).unwrap(), 512.0);
    }
}