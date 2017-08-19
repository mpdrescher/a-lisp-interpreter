use list::List;
use scope::Scope;
use error::Error;
use functions::invalid_types;
use functions::resolve_two_arguments;
use value::Value;

pub fn eq(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "eq")?;
    let is_equal = match (op_1, op_2) {
        (Value::Nil, Value::Nil) => true,
        //(Value::List(list), Value::List(list2)) => list == list2,
        //TODO: make eq for list
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

//TODO lt, gt, le, ge, ne