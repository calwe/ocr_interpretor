use std::collections::HashMap;

use crate::Value;

pub struct SymbolTable {
    symbols: HashMap<String, Value>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn assign_variable(&mut self, ident: String, value: Value) {
        self.symbols.insert(ident, value);
    }

    pub fn get_variable(&mut self, ident: String) -> Value {
        self.symbols.get(&ident).unwrap().clone()
    }
}
