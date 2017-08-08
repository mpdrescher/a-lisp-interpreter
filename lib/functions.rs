use list::List;
use list::resolve;
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
    lambda,
    cond,
    print,
    global
};
use corelib::listops::{
    first,
    last,
    init,
    tail,
    cons,
    len,
    nth
};
use corelib::comp::{
    eq
};

pub fn eval(list: &List, stack: &mut Vec<Scope>) -> Result<Option<Value>, Error> {
    let function = match list.cells().first().unwrap() { //unwrap, because eval checks for empty list
        &Value::Symbol(ref func) => func,
        _ => {
            return Ok(None);
        }
    };
    let result = match &function[..] {
        "lambda" => lambda(list, stack),
        "seq" => prog(list, stack),
        "set" => set(list, stack),
        "global" => global(list, stack),
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
        "init" => init(list, stack),
        "tail" => tail(list, stack),
        "len" => len(list, stack),
        "nth" => nth(list, stack),
        "cons" => cons(list, stack),
        "cond" => cond(list, stack),
        "print" => print(list, stack),
        "eq" => eq(list, stack),
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

//might seem hacky, but is the only way I can use pattern matching
pub fn resolve_argument(list: &List, stack: &mut Vec<Scope>, fn_name: &'static str) -> Result<Value, Error> {
    assert_length(list, 1, fn_name)?;
    Ok(resolve(list.cells().get(1).unwrap().clone(), stack, fn_name)?)
}

pub fn resolve_two_arguments(list: &List, stack: &mut Vec<Scope>, fn_name: &'static str) -> Result<(Value, Value), Error> {
    assert_length(list, 2, fn_name)?;
    Ok((
        resolve(list.cells().get(1).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(2).unwrap().clone(), stack, fn_name)?
    ))
}

/*pub fn resolve_three_arguments(list: &List, stack: &mut Vec<Scope>, fn_name: &'static str) -> Result<(Value, Value, Value), Error> {
    assert_length(list, 3, fn_name)?;
    Ok((
        resolve(list.cells().get(1).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(2).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(3).unwrap().clone(), stack, fn_name)?
    ))
}*/

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