//TODO: function is_numeric
//TODO: function is_uppercase
//TODO: function is_lowercase
//TODO: function is_control
//TODO: function is_alphabetic
//TODO: function to_uppercase
//TODO: function to_lowercase
//TODO: function charval

use value::Value;

pub fn ch_numeric(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "ch_numeric")?;
    match op_1 {
        Value::Char(ch) => {
            return Ok(Value::Boolean(ch.is_numeric()))
        },
        type_1 => {
            invalid_types(vec!(&type_1), "ch_numeric")?;
        }
    }
    Ok(Value::Nil)
}