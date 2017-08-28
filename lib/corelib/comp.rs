use list::List;
use error::Error;
use stack::Stack;
use functions::invalid_types;
use functions::resolve_argument;
use functions::resolve_two_arguments;
use value::Value;

pub fn eq(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "eq")?;
    let is_equal = match (op_1, op_2) {
        (Value::Nil, Value::Nil) => true,
        (Value::List(list), Value::List(list2)) => list == list2,
        (Value::Float(f1), Value::Float(f2)) => f1 == f2,
        (Value::Integer(i1), Value::Integer(i2)) => i1 == i2,
        (Value::Symbol(s1), Value::Symbol(s2)) => s1 == s2,
        (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
        //(Value::Lambda(l1), Value::Lambda(l2)) => l1 == l2,
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "eq")?;
            false
        }
    };
    Ok(Value::Boolean(is_equal))
}

pub fn ne(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "ne")?;
    let is_equal = match (op_1, op_2) {
        (Value::Nil, Value::Nil) => false,
        (Value::List(list), Value::List(list2)) => list != list2,
        (Value::Float(f1), Value::Float(f2)) => f1 != f2,
        (Value::Integer(i1), Value::Integer(i2)) => i1 != i2,
        (Value::Symbol(s1), Value::Symbol(s2)) => s1 != s2,
        (Value::Boolean(b1), Value::Boolean(b2)) => b1 != b2,
        //(Value::Lambda(l1), Value::Lambda(l2)) => l1 != l2,
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "ne")?;
            false
        }
    };
    Ok(Value::Boolean(is_equal))
}

pub fn lt(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "lt")?;
    match (op_1, op_2) {
        (Value::Integer(i_1), Value::Integer(i_2)) => {
            return Ok(Value::Boolean(i_1 < i_2));
        },
        (Value::Integer(i_1), Value::Float(f_2)) => {
            return Ok(Value::Boolean((i_1 as f32) < f_2));
        },
        (Value::Float(f_1), Value::Integer(i_2)) => {
            return Ok(Value::Boolean(f_1 < i_2 as f32));
        },
        (Value::Float(f_1), Value::Float(f_2)) => {
            return Ok(Value::Boolean(f_1 < f_2));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "lt")?;
        }
    };
    Ok(Value::Nil)
}

pub fn gt(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "gt")?;
    match (op_1, op_2) {
        (Value::Integer(i_1), Value::Integer(i_2)) => {
            return Ok(Value::Boolean(i_1 > i_2));
        },
        (Value::Integer(i_1), Value::Float(f_2)) => {
            return Ok(Value::Boolean(i_1 as f32 > f_2));
        },
        (Value::Float(f_1), Value::Integer(i_2)) => {
            return Ok(Value::Boolean(f_1 > i_2 as f32));
        },
        (Value::Float(f_1), Value::Float(f_2)) => {
            return Ok(Value::Boolean(f_1 > f_2));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "gt")?;
        }
    };
    Ok(Value::Nil)
}

pub fn le(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "le")?;
    match (op_1, op_2) {
        (Value::Integer(i_1), Value::Integer(i_2)) => {
            return Ok(Value::Boolean(i_1 <= i_2));
        },
        (Value::Integer(i_1), Value::Float(f_2)) => {
            return Ok(Value::Boolean(i_1 as f32 <= f_2));
        },
        (Value::Float(f_1), Value::Integer(i_2)) => {
            return Ok(Value::Boolean(f_1 <= i_2 as f32));
        },
        (Value::Float(f_1), Value::Float(f_2)) => {
            return Ok(Value::Boolean(f_1 <= f_2));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "le")?;
        }
    };
    Ok(Value::Nil)    
}

pub fn ge(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "ge")?;
    match (op_1, op_2) {
        (Value::Integer(i_1), Value::Integer(i_2)) => {
            return Ok(Value::Boolean(i_1 >= i_2));
        },
        (Value::Integer(i_1), Value::Float(f_2)) => {
            return Ok(Value::Boolean(i_1 as f32 >= f_2));
        },
        (Value::Float(f_1), Value::Integer(i_2)) => {
            return Ok(Value::Boolean(f_1 >= i_2 as f32));
        },
        (Value::Float(f_1), Value::Float(f_2)) => {
            return Ok(Value::Boolean(f_1 >= f_2));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "ge")?;
        }
    };
    Ok(Value::Nil)
}

pub fn and(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "and")?;
    match (op_1, op_2) {
        (Value::Boolean(b1), Value::Boolean(b2)) => {
            return Ok(Value::Boolean(b1 && b2))
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "and")?;
        }
    };
    Ok(Value::Nil)
}

pub fn or(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "or")?;
    match (op_1, op_2) {
        (Value::Boolean(b1), Value::Boolean(b2)) => {
            return Ok(Value::Boolean(b1 || b2))
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "or")?;
        }
    };
    Ok(Value::Nil)
}

pub fn not(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "not")?;
    match op_1 {
        Value::Boolean(b1) => {
            return Ok(Value::Boolean(!b1));
        },
        type_1 => {
            invalid_types(vec!(&type_1), "not")?;
        }
    };
    Ok(Value::Nil)
}

//TODO: lt, gt, le, ge, ne