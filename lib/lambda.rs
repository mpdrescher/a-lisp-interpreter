use list::List;
use value::Value;
use error::Error;
use stack::Stack;

#[derive(Debug, Clone)]
pub struct Lambda {
    param_names: Vec<String>,
    body: List
}

impl Lambda {
    //TODO: check uniqueness of the parameter
    pub fn new(param_names: Vec<String>, body: List) -> Lambda {
        Lambda {
            param_names: param_names,
            body: body
        }
    }

    pub fn param_count(&self) -> usize {
        self.param_names.len()
    }

    pub fn eval_with_trace(&mut self, params: Vec<Value>, stack: &mut Stack, trace: String) -> Result<Value, Error> {
        match self.eval(params, stack) {
            Ok(v) => Ok(v),
            Err(err) => {
                Err(err.add_trace(trace))
            }
        }
    }

    pub fn eval(&mut self, params: Vec<Value>, stack: &mut Stack) -> Result<Value, Error> {
        let expected_len = self.param_names.len();
        let found_len = params.len();
        if expected_len != found_len {
            return Err(Error::new(format!("'lambda': expected {} parameters, found {}.", expected_len, found_len)));
        }
        let param_vec = self.param_names.clone().into_iter().zip(params).collect::<Vec<(String, Value)>>();
        match self.body.eval(stack, Some(param_vec)) {
            Ok(v) => Ok(v),
            err => err
        }
    }

    pub fn from_string(string: String) -> Result<Lambda, Error> {
        let split_index = match string.find('|') {
            Some(index) => index,
            None => return Err(Error::new(format!("lambda definition: expected second '|'.")))
        };
        let (head, body_str) = string.split_at(split_index);
        let params = head.split_whitespace().map(|x| x.to_owned()).collect::<Vec<_>>();
        let body = match List::from_string(body_str[1..].to_owned()) { // 1.. to remove |
            Ok(v) => v,
            Err(mut err) => {
                err = err.add_trace(format!("lambda definition"));
                return Err(err);
            }
        };
        Ok(Lambda::new(params, body))
    }
}