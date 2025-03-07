// Operation API

use crate::bytecode::opcode::Opcode::{ADD, DIVIDE, EXP, MULTIPLY, PUSH, SUBTRACT};

/// Opcodes supported by webwalc bytecode.
pub enum Opcode {
    PUSH,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    EXP,
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
            EXP => 5,
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
            5 => EXP,
            _ => panic!("Unknown opcode {}", byte),
        }
    }
}
