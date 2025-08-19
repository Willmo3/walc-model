use std::error::Error;
use crate::bytecode::opcode::Opcode;
use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, MULTIPLY, PUSH, SUBTRACT, VARWRITE, VARREAD, IDENTIFIER};
use crate::bytecode::stackframe::Binding;
use std::str;
use std::str::Utf8Error;

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
    match state.interpret_scope(&mut stack, &mut root_frame) {
        true => Ok(stack.pop().unwrap()),
        false => Err(state.errors),
    }
}


impl InterpreterState<'_> {
    /*
     * Interpret a single level of program scope, recursing if a new level of depth found.
     * Returns whether frame resulted in bad result.
     */
    fn interpret_scope(&mut self,
                       stack: &mut Vec<f64>,
                       scope_var_binding: &mut Binding) -> bool {

        while self.index < self.code.len() {
            let operation = self.code[self.index];
            self.index += 1; // Skip opcode.

            // Collect errors for this operation.
            let mut iteration_errors = String::new();

            // TODO: unclear that each state can have its own code without some pre-facto analysis.
            // More realistic: bound each scope to names in the higher scope
            // Don't need each scope to have its own code.
            match Opcode::opcode_from_byte(operation) {
                IDENTIFIER => {
                    // Length of identifier.
                    let length = self.code[self.index] as usize;
                    self.index += 1;

                    // TODO: need generic stack of bytes, convert floats from it.
                    let identifier = match str::from_utf8(&self.code[self.index..(self.index + length)]) {
                        Ok(identifier) => identifier,
                        Err(_) => {
                            // Immediately terminate if a UTF conversion error occurs -- the source code is corrupted!
                            self.errors.push_str("Bytecode UTF conversion error. Expected: stream of valid UTF8 bytes for identifier.\n");
                            return false
                        }
                    };

                    // Now, push name and length of identifier onto stack.
                    // stack.extend_from_slice(identifier.bytes())

                }
                VARREAD => {
                    // TODO: implement after stack made polymorphic (multiple data types now on stack, need generic bytes)
                    // let identifier = self.read_identifier();
                    // match identifier {
                    //     Ok(name) => {
                    //         if let Some(value) = scope_var_binding.get_bind(name) {
                    //             stack.push(*value);
                    //         } else {
                    //             iteration_errors.push_str(format!("Variable {} not found!\n", name).as_str());
                    //         }
                    //     }
                    //     Err(e) => {
                    //         self.index += e.valid_up_to();
                    //         iteration_errors.push_str("Bytecode UTF conversion error. Expected: stream of valid UTF8 bytes for identifier.\n");
                    //     }
                    // }
                }
                VARWRITE => {
                    // TODO: implement after stack made polymorphic (multiple data types now on stack)
                }
                PUSH => {
                    let mut immediate_bytes = [0u8; IMM_LEN];
                    immediate_bytes[..IMM_LEN].copy_from_slice(
                        &self.code[self.index..(self.index + IMM_LEN)]);

                    stack.push(f64::from_le_bytes(immediate_bytes));
                    self.index += IMM_LEN; // Read 8-bytes from bytecode value.
                },
                ADD | SUBTRACT | MULTIPLY | DIVIDE | EXP => {
                    if stack.len() < 2 {
                        iteration_errors.push_str("Binary operation attempted with insufficient operands!\n");
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
                                iteration_errors.push_str("Cannot divide by zero.\n");
                            } else {
                                stack.push(left / right)
                            }
                        },
                        EXP => stack.push(left.powf(right)),
                        _ => iteration_errors.push_str(&format!("Unknown binary operation: {}\n", operation)),
                    }
                }
            }
            // Update the list of all errors with the issues in this execution.
            self.errors.push_str(iteration_errors.as_str());
        }

        if stack.len() == 0 {
            self.errors.push_str("No result.\n");
        }

        // If any errors have been detected, an interpretation round is tainted.
        self.errors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::bytecode_interpreter::execute;
    use crate::bytecode::opcode::Opcode;
    use crate::bytecode::opcode::Opcode::{VARWRITE, DIVIDE, MULTIPLY, PUSH, VARREAD, SUBTRACT};

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
    fn test_assign_access() {
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
        code.push(Opcode::byte_from_opcode(&VARWRITE));
        code.push(identifier.len() as u8);
        code.extend_from_slice(identifier.as_bytes());

        // An assignment evaluates to the lval that it produces.
        assert_eq!(execute(&code).unwrap(), -4.0);

        // Additionally, we can dereference that lval.
        code.push(Opcode::byte_from_opcode(&VARREAD));
        code.push(identifier.len() as u8);
        code.extend_from_slice(identifier.as_bytes());

        // This code with a READVAR instruction should also evaluate to -4.0.
        assert_eq!(execute(&code).unwrap(), -4.0);

        // We can also run an arithmetic operation on this result!
        code.push(Opcode::byte_from_opcode(&PUSH));
        code.extend_from_slice(&f64::to_le_bytes(32.0));

        code.push(Opcode::byte_from_opcode(&DIVIDE));

        assert_eq!(execute(&code).unwrap(), -0.125);
    }
}