use error::Error;
use value::Value;
use functions;
use scope::Scope;
use lambda::Lambda;
use stack::Stack;

enum Quoted {
    No,
    Quote,
    Backquote
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn into_cells(self) -> Vec<Value> {
        self.cells
    }

    pub fn from_cells(cells: Vec<Value>) -> List {
        List {
            cells: cells
        }
    }

    pub fn from_string(code: String) -> Result<List, Error> {
        let mut cells = Vec::new();
        let mut buffer = String::new();
        let mut code_iter = code.chars();
        let mut quoted = Quoted::No;
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
                        None => return Err(Error::new(format!("reached end of list code before closing bracket.")))
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
                match quoted {
                    Quoted::No => {
                        cells.push(Value::new_list(List::from_string(inner_buffer)?));
                    },
                    Quoted::Quote => {
                        cells.push(Value::new_list(List::from_string(format!("quote ({})", inner_buffer))?));
                        quoted = Quoted::No;
                    },
                    Quoted::Backquote => {
                        cells.push(Value::new_list(List::from_string(format!("eval ({})", inner_buffer))?));
                        quoted = Quoted::No;
                    }
                };
                // push_to_cells(&mut cells, inner_buffer, &mut quoted)?;
            }
            else if ch == '´' {
                if buffer.len() > 0 {
                    push_to_cells(&mut cells, buffer, &mut quoted)?;
                    buffer = String::new();
                }
                let mut inner_buffer = String::new();
                loop {
                    inner_buffer.push(match code_iter.next() {
                        Some('´') => break,
                        Some(v) => v,
                        None => return Err(Error::new(format!("reached end of list code before closing '´'.")))
                    });
                }
                if (inner_buffer.starts_with("\\") && inner_buffer.len() > 2) || (!inner_buffer.starts_with("\\") && inner_buffer.len() > 1) {
                    return Err(Error::new(format!("the character type can only contain one character.")));
                }
                else if inner_buffer.len() == 0 {
                    return Err(Error::new(format!("a char can not be empty.")));
                }
                cells.push(Value::char_from_string(&inner_buffer[..])?);
            }
            else if ch == '\"' {
                if buffer.len() > 0 {
                    push_to_cells(&mut cells, buffer, &mut quoted)?;
                    buffer = String::new();
                }
                let mut inner_buffer = String::new();
                loop {
                    inner_buffer.push(match code_iter.next() {
                        Some('"') => break,
                        Some(v) => v,
                        None => return Err(Error::new(format!("reached end of list code before closing '\"'.")))
                    });
                }
                let mut string = Vec::new();
                let mut backslash = false;
                for elem in inner_buffer.chars() {
                    match backslash {
                        true => {
                            backslash = false;
                            string.push(Value::char_from_string(&format!("\\{}", elem)[..])?);
                        },
                        false => {
                            string.push(Value::char_from_string(&format!("{}", elem)[..])?);
                        }
                    }
                }
                let mut inner_cells = Vec::new();
                inner_cells.push(Value::Symbol(format!("quote")));
                inner_cells.push(Value::List(List::from_cells(string)));
                cells.push(Value::List(List::from_cells(inner_cells)));
            }
            else if ch == '\'' {
                quoted = Quoted::Quote;
            }
            else if ch == '`' {
                quoted = Quoted::Backquote;
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
            else if ch == '|' { //Lambda
                if cells.len() == 0 && buffer.len() == 0 {
                    let mut inner_buffer = String::new();
                    loop {
                        match code_iter.next() { // consume iterator
                            Some(v) => inner_buffer.push(v),
                            None => break
                        }
                    }
                    cells.push(Value::Lambda(Lambda::from_string(inner_buffer)?));
                }
                else {
                    buffer.push('|');
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
    pub fn eval(&self, stack: &mut Stack, maybe_params: Option<Vec<(String, Value)>>) -> Result<Value, Error> {
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
        let cell_count = self.cells().len();
        if cell_count == 0 {
            stack.pop();
            return Ok(retval)
        }
        match functions::eval(&self, stack)? {
            Some(value) => { //the function has a result without an error
                retval = value;
            }, 
            None => { //there is no builtin function with that name, look for lambdas on the stack or execute the list
                let mut cell_iter = self.cells.iter();
                let name = match cell_iter.next().unwrap() {
                    &Value::Symbol(ref name) => name,
                    &Value::List(ref list) => {
                        if cell_count == 1 {
                            let temp = list.clone().eval(stack, None);
                            stack.pop();
                            return temp;
                        }
                        else { 
                            //evaluate the inner list, append the following items, and evaluate that list
                            //a future change might implement binding values to lambdas similar to
                            //javascripts bind or haskells currying
                            //this would include making the standard functions lambda-like:
                            // -> fold (*) '(1 2 3) = 6
                            let first_elem = list.clone().eval(stack, None)?;
                            let mut temp_cells = vec!(first_elem);
                            for elem in cell_iter { //append remaining
                                temp_cells.push(elem.clone());
                            }
                            let temp = List::from_cells(temp_cells).eval(stack, None);
                            stack.pop();
                            return temp;
                        }
                    },
                    &Value::Lambda(ref lambda) => {
                        stack.pop();
                        if cell_count == 1 {
                            return Ok(Value::Lambda(lambda.clone()));
                        }
                        else {
                            return Err(Error::new(format!("the lambda is not the only element of the list.")));
                        }
                    },
                    value => {
                        stack.pop();
                        return Err(Error::new(format!("expected function name as first list item, found {}.", value.type_str())))
                    }
                };
                let mut lambda = match stack.resolve_variable(name)? {
                    Value::Lambda(lambda) => lambda,
                    _ => {
                        stack.pop();
                        return Err(Error::new(format!("unknown function '{}'.", name)))
                    }
                };
                let param_count = self.cells.len() - 1;
                if param_count != lambda.param_count() {
                    stack.pop();
                    return Err(Error::new(format!("'{}': expected {} function parameters, found {}.", name, lambda.param_count(), param_count)));
                }
                let mut params = Vec::new();
                for i in 1..self.cells.len() {
                    let param = resolve(self.cells.get(i).unwrap().clone(), stack, "[eval]")?;
                    params.push(param);
                }
                retval = match lambda.eval(params, stack) {
                    Ok(v) => v,
                    Err(err) => {
                        return Err(err.add_trace(format!("lambda")));
                    }
                }; 
            }
        };
        stack.pop(); //remove the scope of this function
        Ok(retval)
    }
}

//helper function to tokenize a list string
fn push_to_cells(list: &mut Vec<Value>, buffer: String, quoted: &mut Quoted) -> Result<(), Error> {
    match *quoted {
        Quoted::No => {
            list.push(Value::from_string(buffer));
        },
        Quoted::Quote => {
            list.push(Value::List(List::from_string(format!("quote {}", buffer))?));
            *quoted = Quoted::No;
        },
        Quoted::Backquote => {
            list.push(Value::List(List::from_string(format!("eval {}", buffer))?));
            *quoted = Quoted::No;        
        }
    };
    Ok(())
}

//resolves the parameters a function gets
pub fn resolve(val: Value, stack: &mut Stack, fn_name: &'static str) -> Result<Value, Error> {
    match val {
        Value::List(list) => {
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
            stack.resolve_variable(&symbol)
        },
        Value::Nil => {
            return Ok(Value::List(List::empty()));
        }
        rest => Ok(rest)
    }
}