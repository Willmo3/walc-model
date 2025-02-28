use crate::bytecode::opcode::Opcode;
use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, MULTIPLY, PUSH, SUBTRACT};
use crate::bytecode::stackframe::StackFrame;

const IMM_LEN: usize = 8;

struct InterpreterState<'a> {
    code: &'a Vec<u8>,
    index: usize,
    errors: String,
}

/// Execute a collection of bytes as a walc program.
/// Return f64 result of computation, or all errors encountered.
pub fn execute(bytes: &Vec<u8>) -> Result<f64, String> {
    let mut state = InterpreterState { code: bytes, index: 0, errors: String::new() };
    let mut root_frame = StackFrame::new();

    match interpret_frame(&mut state, &mut root_frame) {
        true => Ok(root_frame.pop_value().unwrap()),
        false => Err(state.errors),
    }
}

/*
 * Interpret a single stack frame, recursing if a new level of depth found.
 * Returns whether frame resulted in bad result.
 */
fn interpret_frame(state: &mut InterpreterState, frame: &mut StackFrame) -> bool {
    while state.index < state.code.len() {
        let operation = state.code[state.index];
        match Opcode::opcode_from_byte(operation) {
            PUSH => {
                state.index += 1; // Skip opcode.

                let mut immediate_bytes = [0u8; IMM_LEN];
                immediate_bytes[..IMM_LEN].copy_from_slice(
                    &state.code[state.index..(state.index + IMM_LEN)]);

                frame.push_value(f64::from_le_bytes(immediate_bytes));
                state.index += 8; // Read 8-bytes from bytecode value.
            },
            ADD | SUBTRACT | MULTIPLY | DIVIDE | EXP => {
                state.index += 1; // Skip opcode

                if frame.stack_size() < 2 {
                    state.errors.push_str("Binary operation attempted with insufficient operands!\n");
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
                            state.errors.push_str("Cannot divide by zero.\n");
                            continue
                        }
                        frame.push_value(left / right)
                    },
                    EXP => frame.push_value(left.powf(right)),
                    _ => state.errors.push_str(&format!("Unknown binary operation: {}\n", operation)),
                }
            }
        }
    }

    if frame.stack_size() == 0 {
        state.errors.push_str("No result.\n");
    }

    // If any errors have been detected, an interpretation round is tainted.
    state.errors.is_empty()
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