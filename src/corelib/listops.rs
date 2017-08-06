use list::List;
use scope::Scope;
use error::Error;
use list::resolve;
use functions::assert_length;
use functions::assert_min_length;
use functions::invalid_types;
use value::Value;

pub fn first(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    assert_length(list, 1, "first")?;
    let op_1 = resolve(list.cells().get(1).unwrap().clone(), stack, "first")?;
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
    assert_length(list, 1, "last")?;
    let op_1 = resolve(list.cells().get(1).unwrap().clone(), stack, "last")?;
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
    assert_length(list, 2, "push")?;
    let op_1 = resolve(list.cells().get(1).unwrap().clone(), stack, "push")?;
    let op_2 = resolve(list.cells().get(2).unwrap().clone(), stack, "push")?;
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