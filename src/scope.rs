use value::Value;
use list::List;
use error::Error;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Scope {
    functions: HashMap<String, Function>,
    vars: HashMap<String, Value>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            functions: HashMap::new(),
            vars: HashMap::new()
        }
    }

    pub fn get_function(&self, name: &String) -> Option<&Function> {
        self.functions.get(name)
    }

    pub fn get_variable(&self, name: &String) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn has_function(&self, name: &String) -> bool {
        self.functions.contains_key(name)
    }

    pub fn has_variable(&self, name: &String) -> bool {
        self.vars.contains_key(name)
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    pub fn set_function(&mut self, name: String, func: Function) -> bool {
        if self.functions.contains_key(&name) {
            return false;
        }
        self.functions.insert(name, func);
		true
	}
}

#[derive(Debug, Clone)]
pub struct Function {
    param_names: Vec<String>,
    body: List
}

impl Function {
    pub fn new(param_names: Vec<String>, body: List) -> Function {
        Function {
            param_names: param_names,
            body: body
        }
    }

    pub fn param_count(&self) -> usize {
        self.param_names.len()
    }

    pub fn eval(&mut self, params: Vec<Value>, stack: &mut Vec<Scope>) -> Result<Value, Error> {
        let param_vec = self.param_names.clone().into_iter().zip(params).collect::<Vec<(String, Value)>>();
        self.body.eval(stack, Some(param_vec))
    }
}