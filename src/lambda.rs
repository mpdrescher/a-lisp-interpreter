use list::List;
use value::Value;
use scope::Scope;
use error::Error;

#[derive(Debug, Clone)]
pub struct Lambda {
    param_names: Vec<String>,
    body: List
}

impl Lambda {
    pub fn new(param_names: Vec<String>, body: List) -> Lambda {
        Lambda {
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