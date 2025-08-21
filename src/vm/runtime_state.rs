use crate::bytecode::opcode::{Opcode, IMM_LEN};
use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, IDENTIFIER, MULTIPLY, PUSH, SUBTRACT, VARWRITE};
use crate::vm::scope_binding::Binding;
use crate::vm::stack::Stack;

/// Runtime state for a program execution.
/// Author: Willmo3
pub(crate) struct RuntimeState<'a> {
    code: &'a Vec<u8>,              // Source code, provided elsewhere.
    pc: usize,                      // Program counter
    pub(crate) stack: Stack,       // Program stack
    pub(crate) errors: String,      // Newline-separated list of errors encountered during execution.
}

impl<'a> RuntimeState<'a> {
    /// Given code, construct a new runtime environment to execute that program.
    pub(crate) fn new(code: &'a Vec<u8>) -> Self {
        Self { code, pc: 0, stack: Stack::new(), errors: String::new() }
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

                    let identifier = match std::str::from_utf8(&self.code[self.pc..(self.pc + length)]) {
                        Ok(identifier) => identifier,
                        Err(_) => {
                            // Immediately terminate if a UTF conversion error occurs -- the source code is corrupted!
                            self.errors.push_str("Bytecode UTF conversion error. Expected: stream of valid UTF8 bytes for identifier.\n");
                            return false
                        }
                    };

                    self.stack.push_identifer(identifier.to_string());
                }
                VARWRITE => {
                    if self.stack.size() < 2 {
                        self.errors.push_str("Not enough values on stack for variable assignment!\n");
                        continue
                    }


                    // Pop the value to write from the stack.
                    let value = match self.stack.pop_float() {
                        Some(value) => { value }
                        None => { self.errors.push_str("Expected float from top of stack, found non-float data.\n"); continue }
                    };

                    // Next, pop the identifier we should write this value to from the stack.
                    let rval = match self.stack.pop_identifier() {
                        Some(id) => { id }
                        None => { self.errors.push_str("Expected identifier from top of stack, found non-identifier data.\n"); continue }
                    };

                    // Assign the new value into our scope binding.
                    scope_binding.set_bind(rval, value);

                    // And push the value we assigned back on the stack
                    self.stack.push_float(value);
                }
                PUSH => {
                    match self.convert_float_from_code() {
                        Ok (float) => {
                            self.stack.push_float(float);
                            self.pc += IMM_LEN;
                        }
                        Err (error) => {
                            self.errors.push_str(&error.to_string());
                        }
                    }
                },
                ADD | SUBTRACT | MULTIPLY | DIVIDE | EXP => {
                    if self.stack.size() < 2 {
                        self.errors.push_str("Binary operation attempted with insufficient operands!\n");
                        continue
                    }

                    // Operands pushed onto stack in reverse order.
                    let right = match self.stack.pop_float() {
                        Some (f) => { f }
                        None => { self.errors.push_str("Found non-float data on stack when expected float.\n"); continue }
                    };
                    // Operands pushed onto stack in reverse order.
                    let left = match self.stack.pop_float() {
                        Some (f) => { f }
                        None => { self.errors.push_str("Found non-float data on stack when expected float."); continue }
                    };

                    match Opcode::opcode_from_byte(operation) {
                        ADD => self.stack.push_float(left + right),
                        SUBTRACT => self.stack.push_float(left - right),
                        MULTIPLY => self.stack.push_float(left * right),
                        DIVIDE => {
                            if right == 0.0 {
                                self.errors.push_str("Cannot divide by zero.\n");
                            } else {
                                self.stack.push_float(left / right)
                            }
                        },
                        EXP => self.stack.push_float(left.powf(right)),
                        _ => self.errors.push_str(&format!("Unknown binary operation: {}\n", operation)),
                    }
                }
            }
        }

        if self.stack.size() == 0 {
            self.errors.push_str("No result.\n");
        }

        // If any errors have been detected, an interpretation round is tainted.
        self.errors.is_empty()
    }

    /// Return the next 8 bytes of code as a string, or a conversion error.
    fn convert_float_from_code(&self) -> Result<f64, String> {
        if self.pc + IMM_LEN >= self.code.len() {
            return Err("Not enough bytes to convert static code into float.\n".to_string());
        }

        let mut immediate_bytes = [0u8; IMM_LEN];
        immediate_bytes[..IMM_LEN].copy_from_slice(&self.code[self.pc..(self.pc + IMM_LEN)]);
        Ok(f64::from_le_bytes(immediate_bytes))
    }
}