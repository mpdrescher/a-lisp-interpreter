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

    pub fn eval(&mut self, params: Vec<Value>, stack: &mut Vec<Scope>) -> Result<Value, Error> {
        let param_vec = self.param_names.clone().into_iter().zip(params).collect::<Vec<(String, Value)>>();
        self.body.eval(stack, Some(param_vec))
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