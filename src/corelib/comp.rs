use list::List;
use scope::Scope;
use error::Error;
use list::resolve;
use functions::assert_length;
use functions::assert_min_length;
use functions::invalid_types;
use value::Value;
use lambda::Lambda;

pub fn eq(list: &List, stack: &mut Vec<Scope>) -> Result<Value, Error> {
    assert_length(list, 2, "eq")?;
    let op_1 = resolve(list.cells().get(1).unwrap().clone(), stack, "eq")?;
    let op_2 = resolve(list.cells().get(2).unwrap().clone(), stack, "eq")?;
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