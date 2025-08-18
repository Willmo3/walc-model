use std::error::Error;
use crate::bytecode::opcode::Opcode;
use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, MULTIPLY, PUSH, SUBTRACT, ASSIGN };
use crate::bytecode::stackframe::Binding;
use std::str;

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
    let mut stack: Vec<f64> = Vec::new();

    // Begin interpreting from the program's root scope, recursively descending lower.
    let mut root_frame = Binding::new();
    match interpret_scope(&mut state, &mut stack, &mut root_frame) {
        true => Ok(stack.pop().unwrap()),
        false => Err(state.errors),
    }
}

/*
 * Interpret a single level of program scope, recursing if a new level of depth found.
 * Returns whether frame resulted in bad result.
 */
fn interpret_scope(state: &mut InterpreterState,
                   stack: &mut Vec<f64>,
                   scope_var_binding: &mut Binding) -> bool {

    while state.index < state.code.len() {
        let operation = state.code[state.index];
        match Opcode::opcode_from_byte(operation) {
            ASSIGN => {
                state.index += 1; // Skip opcode

                // Previous code must have produced value on stack to use.
                if stack.is_empty() {
                    state.errors.push_str("Assignment attempted without value!\n");
                    continue
                }

                // The next field will be the length of the identifier.
                let length = state.code[state.index] as usize;
                state.index += 1;

                // Identifier must be comprised of well-formatted utf8 bytes.
                // We do not support multi-byte characters yet.
                let identifier = str::from_utf8(&state.code[state.index..(state.index + length)]);
                match identifier {
                    Ok(value) => {
                        state.index += value.len();
                        // NOTE: currently, assignments evaluate to whatever value was assigned.
                        // Therefore, we do not bother popping the value from the stack.
                        scope_var_binding.set_bind(String::from(value), *stack.first().unwrap())
                    }
                    Err(e) => {
                        state.index += e.valid_up_to();
                        state.errors.push_str("Bytecode UTF conversion error. Expected: stream of valid UTF8 bytes for identifier.\n");
                        // Skip past the valid parts of the string and continue parsing from the illegal index.
                    }
                }
            },
            PUSH => {
                state.index += 1; // Skip opcode.

                let mut immediate_bytes = [0u8; IMM_LEN];
                immediate_bytes[..IMM_LEN].copy_from_slice(
                    &state.code[state.index..(state.index + IMM_LEN)]);

                stack.push(f64::from_le_bytes(immediate_bytes));
                state.index += IMM_LEN; // Read 8-bytes from bytecode value.
            },
            ADD | SUBTRACT | MULTIPLY | DIVIDE | EXP => {
                state.index += 1; // Skip opcode

                if stack.len() < 2 {
                    state.errors.push_str("Binary operation attempted with insufficient operands!\n");
                    continue
                }

                // Operands pushed onto stack in reverse order.
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                match Opcode::opcode_from_byte(operation) {
                    ADD => stack.push(left + right),
                    SUBTRACT => stack.push(left - right),
                    MULTIPLY => stack.push(left * right),
                    DIVIDE => {
                        if right == 0.0 {
                            state.errors.push_str("Cannot divide by zero.\n");
                            continue
                        }
                        stack.push(left / right)
                    },
                    EXP => stack.push(left.powf(right)),
                    _ => state.errors.push_str(&format!("Unknown binary operation: {}\n", operation)),
                }
            }
        }
    }

    if stack.len() == 0 {
        state.errors.push_str("No result.\n");
    }

    // If any errors have been detected, an interpretation round is tainted.
    state.errors.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::bytecode::bytecode_interpreter::execute;
    use crate::bytecode::opcode::Opcode;
    use crate::bytecode::opcode::Opcode::{ASSIGN, MULTIPLY, SUBTRACT};

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

    #[test]
    fn test_assign() {
        // value_a = 2 - 3 * 2
        let mut code = Vec::new();

        // 2
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));

        // 3 * 2
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(2.0));
        code.push(0u8);
        code.extend_from_slice(&f64::to_le_bytes(3.0));
        code.push(Opcode::byte_from_opcode(&MULTIPLY));

        // 2 - 3 * 2
        code.push(Opcode::byte_from_opcode(&SUBTRACT));

        let identifier = "value_a";
        code.push(Opcode::byte_from_opcode(&ASSIGN));
        code.push(identifier.len() as u8);
        code.extend_from_slice(identifier.as_bytes());

        assert_eq!(execute(&code).unwrap(), -4.0);
    }
}