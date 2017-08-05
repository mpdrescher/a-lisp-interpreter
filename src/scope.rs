use value::Value;
use list::List;
use error::Error;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Scope {
    vars: HashMap<String, Value>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars: HashMap::new()
        }
    }

    pub fn get_variable(&self, name: &String) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn has_variable(&self, name: &String) -> bool {
        self.vars.contains_key(name)
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
}