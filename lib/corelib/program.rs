use list::List;
use scope::Scope;
use error::Error;
use list::resolve;
use functions::assert_length;
use functions::assert_min_length;
use functions::invalid_types;
use functions::resolve_two_arguments;
use functions::resolve_argument;
use value::Value;
use lambda::Lambda;

pub fn lambda(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "lambda")?;
    match (op_1, op_2) {
        (Value::List(params), Value::List(value)) => {
            let mut args = Vec::new();
            for param in params.cells() {
                match param {
                    &Value::Symbol(ref param_str) => {
                        args.push(param_str.clone());
                    },
                    _ => {
                        return Err(Error::new(format!("'lambda': only symbols can be used as function parameters.")));
                    }
                }
            }
            let lambda = Lambda::new(args, value);
            return Ok(Value::new_lambda(lambda));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "lambda")?;
        }
    }
    Ok(Value::Nil)
}

pub fn cond(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    assert_min_length(list, 1, "cond")?;
    for i in 1..list.cells().len() {
        let cell = resolve(list.cells().get(i).unwrap().clone(), stack, "cond")?;
        let inner_list = match cell {
            Value::List(list) => list,
            _ => {
                return Err(Error::new(format!("'cond': expected a list instead of '{}' at index {}.", cell.type_str(), i - 1)));
            }
        };
        if inner_list.cells().len() != 2 {
            return Err(Error::new(format!("'cond': expected a list with two elements at index {}, found one with {}.", i - 1, inner_list.cells().len())));
        }
        let condition_value = resolve(inner_list.cells().get(0).unwrap().clone(), stack, "cond")?;
        let condition = match condition_value {
            Value::Boolean(boolean) => boolean,
            _ => {
                return Err(Error::new(format!("'cond': expected a boolean as the first element at index {}", i - 1)));
            }
        };
        if condition {
            return Ok(resolve(inner_list.cells().get(1).unwrap().clone(), stack, "cond")?);
        }
    }
    Err(Error::new(format!("'cond': no condition was true.")))
}

pub fn prog(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    assert_min_length(list, 2, "prog")?;
    let mut retval = Value::Nil;
    for i in 1..list.cells().len() {
        retval = resolve(list.cells().get(i).unwrap().clone(), stack, "prog")?;
    }
    Ok(retval)
}

pub fn set(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "set")?;
    match (op_1, op_2) {
        (type_1, Value::Symbol(_)) => {
            invalid_types(vec!(&type_1, &Value::Symbol(String::new())), "set")?;
        },
        (Value::Symbol(name), value) => {
            if stack.len() <= 1 {
                return Err(Error::new(format!("'set': no scope above the current one.")));
            }
            let index = stack.len() - 2;
            stack.get_mut(index).unwrap().set_variable(name, value);
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "set")?;
        }
    };
    Ok(Value::Nil)
}

pub fn global(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "global")?;
    match (op_1, op_2) {
        (type_1, Value::Symbol(_)) => {
            invalid_types(vec!(&type_1, &Value::Symbol(String::new())), "global")?;
        },
        (Value::Symbol(name), value) => {
            if stack.len() == 0 {
                return Err(Error::new(format!("'global': no scope found.")));
            }
            stack.get_mut(0).unwrap().set_variable(name, value);
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "global")?;
        }
    };
    Ok(Value::Nil)
}

pub fn quote(list: &List, _stack: &mut Vec<Scope>) -> Result<Value, Error> {
    assert_length(list, 1, "quote")?;
    Ok(list.cells().get(1).unwrap().clone())
}

pub fn print(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    assert_length(list, 1, "print")?;
    let op_1 = resolve_argument(list, stack, "global")?;
    println!("{}", op_1);
    Ok(Value::Nil)
}