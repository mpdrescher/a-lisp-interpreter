use error::Error;
use value::Value;
use functions;
use scope::Scope;

#[derive(Debug, Clone)]
pub struct List {
    cells: Vec<Value>
}

impl List { 
    pub fn empty() -> List {
        List {
            cells: Vec::new()
        }
    }
    
    pub fn cells(&self) -> &Vec<Value> {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut Vec<Value> {
        &mut self.cells
    }

    pub fn from_string(code: String) -> Result<List, Error> {
        let mut cells = Vec::new();
        let mut buffer = String::new();
        let mut code_iter = code.chars();
        let mut quoted = false;
        loop {
            let ch = match code_iter.next() {
                Some(v) => v,
                None => break
            };
            if ch == '(' {
                if buffer.len() > 0 {
                    push_to_cells(&mut cells, buffer, &mut quoted)?;
                    buffer = String::new();
                }
                let mut inner_buffer = String::new();
                let mut depth_counter = 0;
                loop {
                    let inner_ch = match code_iter.next() {
                        Some(v) => v,
                        None => return Err(Error::new(format!("reached end of code before closing bracket.")))
                    };
                    if inner_ch == '(' {
                        depth_counter += 1;
                        inner_buffer.push(inner_ch);
                    }
                    else if inner_ch == ')' {
                        if depth_counter == 0 {
                            break;
                        }
                        else {
                            depth_counter -= 1;
                            inner_buffer.push(inner_ch);
                        }
                    }
                    else {
                        inner_buffer.push(inner_ch);
                    }
                }
                if quoted {
                    cells.push(Value::new_list(List::from_string(format!("quote ({})", inner_buffer))?));
                    quoted = false;
                }
                else {
                    cells.push(Value::new_list(List::from_string(inner_buffer)?));
                }
            }
            else if ch == '\'' {
                quoted = true;
            }
            else if ch == ')' {
                return Err(Error::new(format!("closed bracket before opening it.")));
            }
            else if ch.is_whitespace() {
                if buffer.len() > 0 {
                    push_to_cells(&mut cells, buffer, &mut quoted)?;
                    buffer = String::new();
                }
            }
            else {
                buffer.push(ch);
            }
        }
        if buffer.len() > 0 {
            push_to_cells(&mut cells, buffer, &mut quoted)?;
        }
        Ok(List {
            cells: cells
        })
    }
      
    //TODO: unwind stack on error for when 'try' is implemented
    pub fn eval(&mut self, stack: &mut Vec<Scope>, maybe_params: Option<Vec<(String, Value)>>) -> Result<Value, Error> {
        match maybe_params {
            Some(params) => {
                let mut scope = Scope::new(); //create a new scope with the given parameters
                for param in params {
                    scope.set_variable(param.0, param.1);
                }
                stack.push(scope);
            },
            None => {
                stack.push(Scope::new()); //create a new empty scope for this function
            }
        }
        let mut retval = Value::Nil;
        if self.cells.len() == 0 {
            let _ = stack.pop();
            return Ok(retval)
        }
        match functions::eval(&self, stack)? {
            Some(value) => { //the function has a result without an error
                retval = value;
            }, 
            None => { //there is no builtin function with that name, look for lambdas on the stack or execute the list
                let name = match self.cells.get(0).unwrap() {
                    &Value::Symbol(ref name) => name,
                    &Value::List(ref list) => {
                        return list.clone().eval(stack, None);
                    },
                    value => return Err(Error::new(format!("expected function name as first list item, found {}.", value.type_str())))
                };
                let mut lambda = match resolve_variable(name, stack)? {
                    Value::Lambda(lambda) => lambda,
                    _ => return Err(Error::new(format!("unknown function '{}'.", name)))
                };
                let param_count = self.cells.len() - 1;
                if param_count != lambda.param_count() {
                    return Err(Error::new(format!("'{}': expected {} function parameters, found {}.", name, lambda.param_count(), param_count)));
                }
                let mut params = Vec::new();
                for i in 1..self.cells.len() {
                    let param = resolve(self.cells.get(i).unwrap().clone(), stack, "[eval]")?;
                    params.push(param);
                }
                retval = lambda.eval(params, stack)?;
            }
        };
        let _ = stack.pop(); //remove the scope of this function
        Ok(retval)
    }
}

//helper function to tokenize a list string
fn push_to_cells(list: &mut Vec<Value>, buffer: String, quoted: &mut bool) -> Result<(), Error> {
    if *quoted {
        list.push(Value::List(List::from_string(format!("quote {}", buffer))?));
        *quoted = false;
    }
    else {
        list.push(Value::from_string(buffer));
    }
    Ok(())
}

//resolves the parameters a function gets
pub fn resolve(val: Value, stack: &mut Vec<Scope>, fn_name: &'static str) -> Result<Value, Error> {
    match val {
        Value::List(mut list) => {
            match list.eval(stack, None) {
                Ok(v) => {
                    return Ok(v);
                },
                Err(err) => {
                    return Err(err.add_trace(fn_name.to_owned()));
                }
            }
        },
        Value::Symbol(symbol) => {
            resolve_variable(&symbol, stack)
        },
        Value::Nil => {
            return Ok(Value::List(List::empty()));
        }
        rest => Ok(rest)
    }
}

fn resolve_variable(var: &String, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let mut counter = stack.len() - 1;
    loop {
        let scope = stack.get(counter).unwrap();
        if scope.has_variable(&var) {
            return Ok(scope.get_variable(&var).unwrap().clone()); //TODO: Cloning large variables is bad
        }
        if counter == 0 {
            break;
        }
        counter -= 1;
    }
    Err(Error::new(format!("unknown variable '{}'.", var)))
}