use list::List;
use scope::Scope;
use error::Error;
use functions::{
    resolve_argument,
    resolve_two_arguments,
    invalid_types,
    to_float
};
use value::Value;

pub fn add(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "add")?;
    match (op_1, op_2) {
        (Value::Integer(i_1), Value::Integer(i_2)) => {
            return Ok(Value::Integer(i_1 + i_2));
        },
        (Value::Integer(i_1), Value::Float(f_2)) => {
            return Ok(Value::Float(i_1 as f32 + f_2));
        },
        (Value::Float(f_1), Value::Integer(i_2)) => {
            return Ok(Value::Float(f_1 + i_2 as f32));
        },
        (Value::Float(f_1), Value::Float(f_2)) => {
            return Ok(Value::Float(f_1 + f_2));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "add")?;
        }
    }
    Ok(Value::Nil)
}

pub fn sub(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "sub")?;
    match (op_1, op_2) {
        (Value::Integer(i_1), Value::Integer(i_2)) => {
            return Ok(Value::Integer(i_1 - i_2));
        },
        (Value::Integer(i_1), Value::Float(f_2)) => {
            return Ok(Value::Float(i_1 as f32 - f_2));
        },
        (Value::Float(f_1), Value::Integer(i_2)) => {
            return Ok(Value::Float(f_1 - i_2 as f32));
        },
        (Value::Float(f_1), Value::Float(f_2)) => {
            return Ok(Value::Float(f_1 - f_2));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "sub")?;
        }
    }
    Ok(Value::Nil)
}

pub fn mul(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "mul")?;
    match (op_1, op_2) {
        (Value::Integer(i_1), Value::Integer(i_2)) => {
            return Ok(Value::Integer(i_1 * i_2));
        },
        (Value::Integer(i_1), Value::Float(f_2)) => {
            return Ok(Value::Float(i_1 as f32 * f_2));
        },
        (Value::Float(f_1), Value::Integer(i_2)) => {
            return Ok(Value::Float(f_1 * i_2 as f32));
        },
        (Value::Float(f_1), Value::Float(f_2)) => {
            return Ok(Value::Float(f_1 * f_2));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "mul")?;
        }
    }
    Ok(Value::Nil)
}

pub fn div(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "div")?;
    let val_1 = to_float(op_1);
    let val_2 = to_float(op_2);
    match (val_1, val_2) {
        (Some(v1), Some(v2)) => {
            return Ok(Value::Float(v1 / v2));
        },
        _ => {
            invalid_types(vec!(list.cells().get(1).unwrap(), list.cells().get(2).unwrap()), "div")?;
        }
    }
    Ok(Value::Nil)
}

pub fn sin(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "sin")?;
    let val_1 = to_float(op_1);
    match val_1 {
        Some(f) => {
            return Ok(Value::Float(f.sin()));
        },
        _ => {
            invalid_types(vec!(list.cells().get(1).unwrap()), "sin")?;
        }
    }
    Ok(Value::Nil)
}

pub fn cos(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "cos")?;    
    let val_1 = to_float(op_1);
    match val_1 {
        Some(f) => {
            return Ok(Value::Float(f.cos()));
        },
        _ => {
            invalid_types(vec!(list.cells().get(1).unwrap()), "cos")?;
        }
    }
    Ok(Value::Nil)
}

pub fn tan(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "tan")?;    
    let val_1 = to_float(op_1);
    match val_1 {
        Some(f) => {
            return Ok(Value::Float(f.tan()));
        },
        _ => {
            invalid_types(vec!(list.cells().get(1).unwrap()), "tan")?;
        }
    }
    Ok(Value::Nil)
}