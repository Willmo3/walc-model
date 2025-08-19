use crate::bytecode::opcode::{Opcode, IMM_LEN};
use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, IDENTIFIER, MULTIPLY, PUSH, SUBTRACT, VARREAD, VARWRITE};
use crate::vm::scope_binding::Binding;

/// Runtime state for a program execution.
/// Author: Willmo3
pub(crate) struct RuntimeState<'a> {
    code: &'a Vec<u8>,              // Source code, provided elsewhere.
    pc: usize,                      // Program counter
    pub(crate) stack: Vec<u8>,      // Program stack
    pub(crate) errors: String,                 // Newline-separated list of errors encountered during execution.
}

impl<'a> RuntimeState<'a> {
    /// Given code, construct a new runtime environment to execute that program.
    pub(crate) fn new(code: &'a Vec<u8>) -> Self {
        Self { code, pc: 0, stack: Vec::new(), errors: String::new() }
    }

    /*
     * Interpret a single level of program scope, recursing if a new level of depth found.
     * Returns whether frame resulted in bad result.
     */
    pub(crate) fn interpret_scope(&mut self, scope_binding: &mut Binding) -> bool {
        while self.pc < self.code.len() {
            let operation = self.code[self.pc];
            self.pc += 1; // Skip opcode.

            match Opcode::opcode_from_byte(operation) {
                IDENTIFIER => {
                    // Length of identifier.
                    let length = self.code[self.pc] as usize;
                    self.pc += 1;

                    // TODO: need generic stack of bytes, convert floats from it.
                    let identifier = match std::str::from_utf8(&self.code[self.pc..(self.pc + length)]) {
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
                    match self.float_from_code() {
                        Ok (float) => {
                            self.stack.extend_from_slice(&float.to_le_bytes());
                            self.pc += IMM_LEN;
                        }
                        Err (error) => {
                            self.errors.push_str(&error.to_string());
                        }
                    }
                },
                ADD | SUBTRACT | MULTIPLY | DIVIDE | EXP => {
                    if self.stack.len() < 2 * IMM_LEN {
                        self.errors.push_str("Binary operation attempted with insufficient operands!\n");
                        continue
                    }

                    // Operands pushed onto stack in reverse order.
                    let right = match self.pop_float_from_stack() {
                        Ok (float) => { float }
                        Err (e) => { self.errors.push_str(&*e); continue }
                    };
                    let left = match self.pop_float_from_stack() {
                        Ok (float) => { float }
                        Err (e) => { self.errors.push_str(&*e); continue }
                    };

                    match Opcode::opcode_from_byte(operation) {
                        ADD => self.push_float_to_stack(left + right),
                        SUBTRACT => self.push_float_to_stack(left - right),
                        MULTIPLY => self.push_float_to_stack(left * right),
                        DIVIDE => {
                            if right == 0.0 {
                                self.errors.push_str("Cannot divide by zero.\n");
                            } else {
                                self.push_float_to_stack(left / right)
                            }
                        },
                        EXP => self.push_float_to_stack(left.powf(right)),
                        _ => self.errors.push_str(&format!("Unknown binary operation: {}\n", operation)),
                    }
                }
            }
        }

        if self.stack.len() == 0 {
            self.errors.push_str("No result.\n");
        }

        // If any errors have been detected, an interpretation round is tainted.
        self.errors.is_empty()
    }

    // TODO: separate data structure for stack.

    /// Pop the top of the program stack, turning it into code, if possible.
    /// If not, return error message.
    pub(crate) fn pop_float_from_stack(&mut self) -> Result<f64, String> {
        if self.stack.len() < IMM_LEN {
            return Err("Expected 64-bit float on stack, found too few bytes.\n".to_string());
        }

        // pop top IMM_LEN bytes from stack.
        let mut immediate_bytes = [0u8; IMM_LEN];
        immediate_bytes[..IMM_LEN].copy_from_slice(
            &self.stack[self.stack.len()-IMM_LEN..self.stack.len()]);
        self.stack.truncate(self.stack.len()-IMM_LEN);

        Ok(f64::from_le_bytes(immediate_bytes))
    }

    fn push_float_to_stack(&mut self, float: f64) {
        self.stack.extend_from_slice(float.to_le_bytes().as_slice());
    }

    /// Return the next 8 bytes of code as a string, or a conversion error.
    fn float_from_code(&self) -> Result<f64, String> {
        if self.pc + IMM_LEN >= self.code.len() {
            return Err("Not enough bytes to convert static code into float.\n".to_string());
        }

        let mut immediate_bytes = [0u8; IMM_LEN];
        immediate_bytes[..IMM_LEN].copy_from_slice(&self.code[self.pc..(self.pc + IMM_LEN)]);
        Ok(f64::from_le_bytes(immediate_bytes))
    }
}