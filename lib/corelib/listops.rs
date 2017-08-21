use list::List;
use scope::Scope;
use error::Error;
use functions::invalid_types;
use functions::resolve_argument;
use functions::resolve_two_arguments;
use functions::resolve_three_arguments;
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

pub fn last(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "last")?;
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

pub fn init(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "init")?;
    match op_1 {
        Value::List(mut list) => {
            if list.cells().len() > 0 {
                let len = list.cells().len();
                list.cells_mut().remove(len - 1);
                return Ok(Value::List(list));
            }
            else {
                return Ok(Value::Nil);
            }
        },
        type_1 => {
            invalid_types(vec!(&type_1), "init")?;
        }
    }
    Ok(Value::Nil)
}

pub fn tail(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "tail")?;
    match op_1 {
        Value::List(mut list) => {
            if list.cells().len() > 0 {
                list.cells_mut().remove(0);
                return Ok(Value::List(list));
            }
            else {
                return Ok(Value::Nil);
            }
        },
        type_1 => {
            invalid_types(vec!(&type_1), "tail")?;
        }
    }
    Ok(Value::Nil)
}

pub fn len(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "len")?;
    match op_1 {
        Value::List(list) => {
            return Ok(Value::Integer(list.cells().len() as i32));
        },
        type_1 => {
            invalid_types(vec!(&type_1), "len")?;
        }
    }
    Ok(Value::Nil)
}

pub fn nth(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "nth")?;
    match (op_1, op_2) {
        (Value::Integer(index), Value::List(list)) => {
            if index < 0 {
                return Err(Error::new_with_origin("nth", format!("index must be non-negative.")));
            }
            else if list.cells().len() == 0 || index >= list.cells().len() as i32 { //TODO: rethink this
                return Ok(Value::Nil);
            }
            return Ok(list.cells().get(index as usize).unwrap().clone());
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "nth")?;
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

//TODO: add function 'shape' which gives the nested size of a nested list like apls shape
//TODO: add function 'splitat'
//TODO: add function 'split'

//TODO: add error trace to an error when a lamdba fails

pub fn map(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "map")?;
    match (op_1, op_2) {
        (Value::Lambda(mut lambda), Value::List(list)) => {
            let mut result = Vec::new();
            for value in list.into_cells().into_iter() {
                result.push(lambda.eval_with_trace(vec!(value), stack, format!("map"))?);
            }
            return Ok(Value::List(List::new_with_cells(result)));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "map")?;
        }
    }
    Ok(Value::Nil)
}

pub fn fold(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2, op_3) = resolve_three_arguments(list, stack, "fold")?;
    match (op_1, op_2, op_3) {
        (first, Value::Lambda(mut lambda), Value::List(list)) => {
            let mut acc;
            if list.cells().len() == 0 {
                return Ok(Value::Nil);
            }
            acc = first;
            for elem in list.into_cells().into_iter() {
                acc = lambda.eval_with_trace(vec!(acc, elem), stack, format!("fold"))?;
            }
            return Ok(acc);
        },
        (type_1, type_2, type_3) => {
            invalid_types(vec!(&type_1, &type_2, &type_3), "fold")?;
        }
    }
    Ok(Value::Nil)
}

pub fn any(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "any")?;
    match (op_1, op_2) {
        (Value::Lambda(mut lambda), Value::List(list)) => {
            let mut result = false;
            let mut index_counter = 0;
            for elem in list.into_cells().into_iter() {
                let elem_result = lambda.eval_with_trace(vec!(elem), stack, format!("any"))?;
                match elem_result {
                    Value::Boolean(true) => {
                        result = true;
                    },
                    Value::Boolean(false) => {},
                    _ => {
                        return Err(Error::new_with_origin("any", format!("expected boolean at index {}.", index_counter)));
                    }
                }
                index_counter += 1;
            }
            return Ok(Value::Boolean(result));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "any")?;
        }
    }
    Ok(Value::Nil)
}

pub fn all(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "all")?;
    match (op_1, op_2) {
        (Value::Lambda(mut lambda), Value::List(list)) => {
            let mut result = true;
            if list.cells().len() == 0 {
                return Ok(Value::Boolean(false));
            }
            let mut index_counter = 0;
            for elem in list.into_cells().into_iter() {
                let elem_result = lambda.eval_with_trace(vec!(elem), stack, format!("all"))?;
                match elem_result {
                    Value::Boolean(false) => {
                        result = false;
                    },
                    Value::Boolean(true) => {},
                    _ => {
                        return Err(Error::new_with_origin("all", format!("expected boolean at index {}.", index_counter)));
                    }
                }
                index_counter += 1;
            }
            return Ok(Value::Boolean(result));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "all")?;
        }
    }
    Ok(Value::Nil)
}

//TODO: function zip
//TODO: function 'any'
//TODO: function 'all'
//TODO: function 'filter'
//TODO: function 'rev'
//TODO: function 'expand'