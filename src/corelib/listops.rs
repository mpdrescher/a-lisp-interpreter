use list::List;
use scope::Scope;
use error::Error;
use functions::invalid_types;
use functions::resolve_argument;
use functions::resolve_two_arguments;
use value::Value;

pub fn first(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "first")?;
    match op_1 {
        Value::List(list) => {
            if list.cells().len() > 0 {
                return Ok(list.cells().get(0).unwrap().clone());
            }
            else {
                return Ok(Value::Nil);
            }
        },
        type_1 => {
            invalid_types(vec!(&type_1), "first")?;
        }
    }
    Ok(Value::Nil)
}

pub fn rest(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "rest")?;
    match op_1 {
        Value::List(list) => {
            if list.cells().len() > 0 {
                let index = list.cells().len() - 1;
                return Ok(list.cells().get(index).unwrap().clone());
            }
            else {
                return Ok(Value::Nil);
            }
        },
        type_1 => {
            invalid_types(vec!(&type_1), "last")?;
        }
    }
    Ok(Value::Nil)
}

pub fn cons(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "cons")?;
    match (op_1, op_2) {
        (val, Value::List(mut list)) => {
            list.cells_mut().insert(0, val);
            return Ok(Value::List(list));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "push")?;
        }
    }
    Ok(Value::Nil)
}