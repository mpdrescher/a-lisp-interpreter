use list::List;
use error::Error;
use value::Value;
use scope::Scope;

use corelib::math::{
    add,
    sub,
    mul,
    div,
    sin,
    cos,
    tan
};
use corelib::program::{
    set,
    prog,
    quote,
    lambda
};
use corelib::listops::{
    first,
    last,
    push
};

pub fn eval(list: &List, stack: &mut Vec<Scope>) -> Result<Option<Value>, Error> {
    let function = match list.cells().first().unwrap() { //unwrap, because eval checks for empty list
        &Value::Word(ref func) => func,
        rest => {
            return Err(Error::new(format!("expected function in the first cell, found {}.", rest.type_str())))
        }
    };
    let result = match &function[..] {
        "lambda" => lambda(list, stack),
        "seq" => prog(list, stack),
        "set" => set(list, stack),
        "quote" => quote(list, stack),
        "add" => add(list, stack),
        "sub" | "-" => sub(list, stack),
        "mul" => mul(list, stack),
        "div" | "/" => div(list, stack),
        "sin" => sin(list, stack),
        "cos" => cos(list, stack),
        "tan" => tan(list, stack),
        "first" => first(list, stack),
        "last" => last(list, stack),
        "push" => push(list, stack),
        _ => {
            return Ok(None)
        }
    };
    match result {
        Ok(value) => Ok(Some(value)),
        Err(err) => Err(err)
    }
}

pub fn invalid_types(types: Vec<&Value>, fn_name: &'static str) -> Result<(), Error> {
    let mut type_str = String::new();
    for t in types {
        if type_str.len() != 0 {
            type_str.push(',');
            type_str.push(' ');
        }
        type_str.push_str(t.type_str());
    }
    Err(Error::new(format!("invalid types in '{}': {}", fn_name, type_str)))
}

pub fn assert_min_length(list: &List, length: usize, fn_name: &'static str) -> Result<(), Error> {
    let len = list.cells().len() - 1;
    if len < length {
        return Err(Error::new(format!("'{}': requires {} parameters, found {}.", fn_name, length, len)));
    }
    return Ok(())
}

pub fn assert_length(list: &List, length: usize, fn_name: &'static str) -> Result<(), Error> {
    let len = list.cells().len() - 1;
    if len != length {
        return Err(Error::new(format!("{}: requires {} parameters, found {}.", fn_name, length, len)));
    }
    return Ok(())
}

pub fn to_float(value: Value) -> Option<f32> {
    match value {
        Value::Integer(i) => Some(i as f32),
        Value::Float(f) => Some(f),
        _ => None
    }
}