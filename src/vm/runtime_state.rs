use crate::bytecode::opcode::{Opcode, IMM_LEN};
use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, IDENTIFIER, MULTIPLY, PUSH, SUBTRACT, VARWRITE};
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

                    let identifier = match std::str::from_utf8(&self.code[self.pc..(self.pc + length)]) {
                        Ok(identifier) => identifier,
                        Err(_) => {
                            // Immediately terminate if a UTF conversion error occurs -- the source code is corrupted!
                            self.errors.push_str("Bytecode UTF conversion error. Expected: stream of valid UTF8 bytes for identifier.\n");
                            return false
                        }
                    };

                    self.push_identifier_to_stack(identifier);
                }
                VARWRITE => {
                    // Pop the value to write from the stack.
                    let value = match self.pop_float_from_stack() {
                        Ok(value) => { value }
                        Err(e) => { self.errors.push_str(&*e); return false }
                    };

                    // Next, pop the identifier we should write this value to from the stack.
                    let rval = match self.pop_identifier_from_stack() {
                        Ok(identifier) => { identifier }
                        Err(e) => {
                            self.errors.push_str(&*e);
                            // If no identifier was found on the stack, and we didn't already catch this structural error during parsing, no further options possible.
                            return false
                        }
                    };

                    // Assign the new value into our scope binding.
                    scope_binding.set_bind(rval, value);

                    // And push the value we assigned back on the stack
                    self.push_float_to_stack(value);
                }
                PUSH => {
                    match self.convert_float_from_code() {
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

    /// Push an identifier onto the stack, following with its length as a single byte.
    fn push_identifier_to_stack(&mut self, identifier: &str) {
        let length = identifier.len();
        if length > u8::MAX as usize {
            panic!("Identifier length exceeds the maximum allowed length.");
        }
        self.stack.extend_from_slice(&identifier.as_bytes());
        self.stack.push(length as u8);
    }

    fn pop_identifier_from_stack(&mut self) -> Result<String, String> {
        let length = match self.stack.pop() {
            Some(length) => { length as usize }
            None => { return Err("Expected identifier on stack, found none.".to_string()) }
        };

        if self.stack.len() < length {
            return Err(format!("Expected identifier of length {} on top of stack, but found too few bytes on stack!\n", self.stack.len()));
        }

        let bytes = &self.stack[self.stack.len() - length..self.stack.len()];
        match std::str::from_utf8(bytes) {
            Ok(identifier) => { Ok(identifier.to_string()) }
            Err(e) => { Err(e.to_string()) }
        }
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