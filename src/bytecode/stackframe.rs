// Individual frame of Walc execution.
// Author: Will Morris

use std::collections::HashMap;

pub(crate) struct Binding {
    parent: Option<Box<Binding>>,
    bindings: HashMap<String, f64>,
}

impl Binding {
    pub fn new() -> Binding {
        Binding { parent: None, bindings: HashMap::new() }
    }
}

/// Binding API
impl Binding {
    fn set_bind(&mut self, binding_name: String, bind_value: f64) {
        self.bindings.insert(binding_name, bind_value);
    }
    fn get_bind(&self, binding_name: &str) -> Option<&f64> {
        self.bindings.get(binding_name)
    }
}