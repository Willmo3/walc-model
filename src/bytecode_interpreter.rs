use crate::bytecode_interpreter::Opcode::{ADD, DIVIDE, MULTIPLY, PUSH, SUBTRACT};

/// Bytecode interpreter for the walc programming language.
/// Author: Will Morris
enum Opcode {
    PUSH,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}
const OPCODES: &'static[Opcode; 5] = &[PUSH, ADD, SUBTRACT, MULTIPLY, DIVIDE];
const IMM_LEN: usize = 8;

/// Interpret a collection of bytes as a walc program.
/// Return f64 result of computation.
pub fn interpret(bytes: &Vec<u8>) -> f64 {
    let mut index = 0;
    let mut stack: Vec<f64> = vec![0.0];

    while index < bytes.len() {
        let operation = bytes[index];
        match OPCODES[operation as usize] {
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
                    panic!("Binary operation attempted with insufficient operands!")
                }

                // Operands pushed onto stack in reverse order.
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                match OPCODES[operation as usize] {
                    ADD => stack.push(left + right),
                    SUBTRACT => stack.push(left - right),
                    MULTIPLY => stack.push(left * right),
                    // TODO: robust checking for 0, perhaps using optional type?
                    DIVIDE => stack.push(left / right),
                    _ => panic!("Unknowmn binary operation: {}", operation),
                }
            }
        }
    }

    match stack.pop() {
        None => { panic!("Expected return value but none found -- likely internal error!") }
        Some(val) => { val }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode_interpreter::interpret;

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
        assert_eq!(interpret(&code), f64::MAX);
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

        assert_eq!(interpret(&code), -1.0);
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

        assert_eq!(interpret(&code), 256.0);
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

        assert_eq!(interpret(&code), 0.5);
    }
}