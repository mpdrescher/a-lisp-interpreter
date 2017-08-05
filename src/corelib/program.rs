use list::List;
use scope::Scope;
use error::Error;
use list::resolve;
use functions::assert_length;
use functions::assert_min_length;
use functions::invalid_types;
use scope:: Function;
use value::Value;

pub fn defun(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    assert_length(list, 3, "defun")?;
    let op_1 = list.cells().get(1).unwrap().clone();
    let op_2 = list.cells().get(2).unwrap().clone();
    let op_3 = list.cells().get(3).unwrap().clone();
    match (op_1, op_2, op_3) {
        (Value::Word(name), Value::List(params), Value::List(value)) => {
            let mut args = Vec::new();
            for param in params.cells() {
                match param {
                    &Value::Word(ref param_str) => {
                        args.push(param_str.clone());
                    },
                    _ => {
                        return Err(Error::new(format!("'defun': only words can be used as function parameters.")));
                    }
                }
            }
            if stack.len() <= 1 {
                return Err(Error::new(format!("'defun': no scope above the current one.")));
            }
            let index = stack.len() - 2;
            if stack.get(index).unwrap().has_function(&name) {
                return Err(Error::new(format!("'defun': a function with the name '{}' already exists.", name)));
            }
            let function = Function::new(args, value);
            let result = stack.get_mut(index).unwrap().set_function(name, function);
            if !result {
                panic!("function already exists. this should have been checked.");
            }
        },
        (type_1, type_2, type_3) => {
            invalid_types(vec!(&type_1, &type_2, &type_3), "defun")?;
        }
    }
    Ok(Value::Nil)
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
    assert_length(list, 2, "set")?;
    let op_1 = list.cells().get(1).unwrap().clone();
    let op_2 = resolve(list.cells().get(2).unwrap().clone(), stack, "set")?;
    match (op_1, op_2) {
        (type_1, Value::Word(_)) => {
            invalid_types(vec!(&type_1, &Value::Word(String::new())), "set")?;
        },
        (Value::Word(name), value) => {
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

pub fn quote(list: &List, _stack: &mut Vec<Scope>) -> Result<Value, Error> {
    assert_length(list, 1, "quote")?;
    Ok(list.cells().get(1).unwrap().clone())
}