// StackData wraps primitive values with runtime type hints.
enum StackData {
    Float(f64),
    Identifier(String),
}

pub struct Stack {
    data: Vec<StackData>,
}

// General helpers
impl Stack {
    pub fn new() -> Stack {
        Stack { data: vec![] }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

// Push functions wrap values with internal type information.
impl Stack {
    pub fn push_float(&mut self, value: f64) {
        self.data.push(StackData::Float(value));
    }

    pub fn push_identifer(&mut self, name: String) {
        self.data.push(StackData::Identifier(name));
    }
}

// Pop functions attempt to remove a value of the given type from the top of the stack, if possible.
impl Stack {
    pub fn pop_float(&mut self) -> Option<f64> {
        if let Some(StackData::Float(v)) = self.data.pop() {
            Some(v)
        } else {
            None
        }
    }

    pub fn pop_identifier(&mut self) -> Option<String> {
        if let Some(StackData::Identifier(v)) = self.data.pop() {
            Some(v)
        } else {
            None
        }
    }
}
