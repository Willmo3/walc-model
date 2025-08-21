use crate::vm::scope_binding::Binding;
use crate::vm::runtime_state::RuntimeState;

const IMM_LEN: usize = 8;

/// Execute a collection of bytes as a walc program.
/// Return f64 result of computation, or all errors encountered.
pub fn execute(bytes: &Vec<u8>) -> Result<f64, String> {
    let mut runtime = RuntimeState::new(bytes);

    // Begin interpreting from the program's root scope, recursively descending lower.
    let mut root_frame = Binding::new();
    match runtime.interpret_scope(&mut root_frame) {
        true => Ok(runtime.stack.pop_float().unwrap()),
        false => Err(runtime.errors),
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::interpreter::execute;
    use crate::bytecode::opcode::Opcode;
    use crate::bytecode::opcode::Opcode::{VARWRITE, DIVIDE, MULTIPLY, PUSH, SUBTRACT, IDENTIFIER};

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
        // TODO: fix ordering bugs.
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

        // x = 2 - 3 * 2
        let identifier = "value_a";
        code.push(Opcode::byte_from_opcode(&IDENTIFIER));
        code.push(identifier.len() as u8);
        code.extend_from_slice(identifier.as_bytes());

        code.push(Opcode::byte_from_opcode(&VARWRITE));

        // An assignment evaluates to the lval that it produces.
        assert_eq!(execute(&code).unwrap(), -4.0);
        //
        // // Additionally, we can dereference that lval.
        // code.push(Opcode::byte_from_opcode(&IDENTIFIER));
        // code.push(identifier.len() as u8);
        // code.extend_from_slice(identifier.as_bytes());
        //
        // // This code with a READVAR instruction should also evaluate to -4.0.
        // assert_eq!(execute(&code).unwrap(), -4.0);
        //
        // // We can also run an arithmetic operation on this result!
        // code.push(Opcode::byte_from_opcode(&PUSH));
        // code.extend_from_slice(&f64::to_le_bytes(32.0));
        //
        // code.push(Opcode::byte_from_opcode(&DIVIDE));
        //
        // assert_eq!(execute(&code).unwrap(), -0.125);
    }
}