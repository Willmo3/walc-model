use std::collections::HashMap;

/// Represents an individual program scope, such as in a function, and its variable bindings. 
/// NOTE: currently, the only atoms are primitives.
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
    pub(crate) fn set_bind(&mut self, binding_name: String, bind_value: f64) {
        self.bindings.insert(binding_name, bind_value);
    }
    pub(crate) fn get_bind(&self, binding_name: &str) -> Option<&f64> {
        self.bindings.get(binding_name)
    }
}