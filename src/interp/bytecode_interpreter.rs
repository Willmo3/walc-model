use crate::interp::bytecode_interpreter::Opcode::{ADD, DIVIDE, MULTIPLY, PUSH, SUBTRACT};

// Operation API

/// Opcodes supported by webwalc bytecode.
enum Opcode {
    PUSH,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

// Opcode to byte translation
impl Opcode {
    /// Given an Opcode, convert it to its byte representation.
    pub fn byte_from_opcode(&self) -> u8 {
        match self {
            PUSH => 0,
            ADD => 1,
            SUBTRACT => 2,
            MULTIPLY => 3,
            DIVIDE => 4,
        }
    }

    /// Given a byte, convert it to its opcode representation.
    /// Or panic, if unsupported opcode.
    pub fn opcode_from_byte(byte: u8) -> Self {
        match byte {
            0 => PUSH,
            1 => ADD,
            2 => SUBTRACT,
            3 => MULTIPLY,
            4 => DIVIDE,
            _ => panic!("Unknown opcode {}", byte),
        }
    }
}

const IMM_LEN: usize = 8;

/// Interpret a collection of bytes as a walc program.
/// Return f64 result of computation.
pub fn interpret(bytes: &Vec<u8>) -> Result<f64, String> {
    let mut index = 0;
    let mut stack: Vec<f64> = Vec::new();
    let mut errors = String::new();

    while index < bytes.len() {
        let operation = bytes[index];
        match Opcode::opcode_from_byte(operation) {
            PUSH => {
                index += 1; // Skip opcode.

                let mut immediate_bytes = [0u8; IMM_LEN];
                immediate_bytes[..IMM_LEN].copy_from_slice(&bytes[index..(index + IMM_LEN)]);
                stack.push(f64::from_le_bytes(immediate_bytes));
                index += 8; // Read 8-bytes from bytecode value.
            },
            ADD | SUBTRACT | MULTIPLY | DIVIDE => {
                index += 1; // Skip opcode

                if stack.len() < 2 {
                    errors.push_str("Binary operation attempted with insufficient operands!\n");
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
                            errors.push_str("Cannot divide by zero.\n");
                            continue
                        }
                        stack.push(left / right)
                    },
                    _ => errors.push_str(&format!("Unknown binary operation: {}\n", operation)),
                }
            }
        }
    }

    if stack.is_empty() {
        errors.push_str("No result.\n");
    }

    if errors.is_empty() {
        Ok(stack.pop().unwrap())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use crate::interp::bytecode_interpreter::interpret;

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
        assert_eq!(interpret(&code).unwrap(), f64::MAX);
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

        assert_eq!(interpret(&code).unwrap(), -1.0);
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

        assert_eq!(interpret(&code).unwrap(), 256.0);
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

        assert_eq!(interpret(&code).unwrap(), 0.5);
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

        assert_eq!(interpret(&code), Err("Cannot divide by zero.\nNo result.\n".to_string()));
    }
}