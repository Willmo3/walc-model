// Individual frame of Walc execution.
// Author: Will Morris

use std::collections::HashMap;

pub(crate) struct StackFrame {
    parent: Option<Box<StackFrame>>,
    bindings: HashMap<String, f64>,
    stack: Vec<f64>,
}

impl StackFrame {
    pub fn new() -> StackFrame {
        StackFrame { parent: None, bindings: HashMap::new(), stack: Vec::new() }
    }
}

/// Binding API
impl StackFrame {
    fn set_bind(&mut self, binding_name: String, bind_value: f64) {
        self.bindings.insert(binding_name, bind_value);
    }
    fn get_bind(&self, binding_name: &str) -> Option<&f64> {
        self.bindings.get(binding_name)
    }
}

/// Variable stack API
impl StackFrame {
    pub(crate) fn push_value(&mut self, value: f64) {
        self.stack.push(value);
    }
    pub(crate) fn pop_value(&mut self) -> Option<f64> {
        self.stack.pop()
    }
    pub(crate) fn stack_size(&self) -> usize {
        self.stack.len()
    }
}