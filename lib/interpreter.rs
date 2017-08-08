use value::Value;
use error::Error;
use list::List;
use scope::Scope;

pub struct Interpreter {
    global: Scope
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            global: Scope::new()
        }
    }

    pub fn eval(&mut self, code: String) -> Result<Value, Error> {
        let mut list = List::from_string(code)?;
        let mut stack = vec!(self.global.clone());
        let result = list.eval(&mut stack, None);
        self.global = stack.into_iter().next().unwrap();
        result
    }
}
