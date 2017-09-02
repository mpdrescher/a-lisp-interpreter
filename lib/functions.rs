use list::List;
use list::resolve;
use error::Error;
use value::Value;
use stack::Stack;
use ::FLOAT;

use corelib::math::{
    add,
    sub,
    mul,
    div,
    sin,
    cos,
    tan,
    count,
    modulo
};
use corelib::program::{
    set,
    seq,
    quote,
    lambda,
    cond,
    printfmt,
    print,
    global,
    while_loop,
    spawn,
    eval as eval_fn,
    puts,
    putsln,
    throw,
    try,
    type_fn,
    try_rename,
    format,
};
use corelib::listops::{
    first,
    last,
    init,
    tail,
    cons,
    len,
    nth,
    map,
    fold,
    expand,
    filter,
    any,
    all,
    append,
    unique,
    find,
    split_at,
    combine,
    intersect,
    zip,
    rev,
    sort
};
use corelib::comp::{
    eq,
    ne,
    lt,
    gt,
    le,
    ge,
    and,
    not,
    or
};

pub fn eval(list: &List, stack: &mut Stack) -> Result<Option<Value>, Error> {
    let function = match list.cells().first().unwrap() { //unwrap, because eval checks for empty list
        &Value::Symbol(ref func) => func,
        _ => {
            return Ok(None);
        }
    };
    let result = match &function[..] {
        "lambda" => lambda(list, stack),
        "seq" => seq(list, stack),
        "set" | "$" => set(list, stack),
        "global" => global(list, stack),
        "quote" => quote(list, stack),
        "add" | "+" => add(list, stack),
        "sub" | "-" => sub(list, stack),
        "mul" | "*" => mul(list, stack),
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
        "printfmt" => printfmt(list, stack),
        "print" => print(list, stack),
        "eq" | "=" => eq(list, stack),
        "ne" | "!=" => ne(list, stack),
        "lt" | "<" => lt(list, stack),
        "gt" | ">" => gt(list, stack),
        "le" | "<=" => le(list, stack),
        "ge" | ">=" => ge(list, stack),
        "map" | "%" => map(list, stack),
        "fold" | "\\" => fold(list, stack),
        "any" => any(list, stack),
        "all" => all(list, stack),
        "filter" | "_" => filter(list, stack),
        "count" | ".." => count(list, stack),
        "mod" => modulo(list, stack),
        "and" => and(list, stack),
        "or" => or(list, stack),
        "not" => not(list, stack),
        "while" => while_loop(list, stack),
        "spawn" => spawn(list, stack),
        "eval" => eval_fn(list, stack),
        "append" => append(list, stack),
        "unique" => unique(list, stack),
        "expand" => expand(list, stack),
        "puts" => puts(list, stack),
        "putsln" => putsln(list, stack),
        "throw" => throw(list, stack),
        "try" => try(list, stack),
        "type" => type_fn(list, stack),
        "try_rename" => try_rename(list, stack),
        "format" => format(list, stack),
        "find" => find(list, stack),
        "split_at" => split_at(list, stack),
        "combine" => combine(list, stack),
        "intersect" => intersect(list, stack),
        "zip" => zip(list, stack),
        "rev" => rev(list, stack),
        "sort" => sort(list, stack),
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
pub fn resolve_argument(list: &List, stack: &mut Stack, fn_name: &'static str) -> Result<Value, Error> {
    assert_length(list, 1, fn_name)?;
    Ok(resolve(list.cells().get(1).unwrap().clone(), stack, fn_name)?)
}

pub fn resolve_two_arguments(list: &List, stack: &mut Stack, fn_name: &'static str) -> Result<(Value, Value), Error> {
    assert_length(list, 2, fn_name)?;
    Ok((
        resolve(list.cells().get(1).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(2).unwrap().clone(), stack, fn_name)?
    ))
}

pub fn resolve_three_arguments(list: &List, stack: &mut Stack, fn_name: &'static str) -> Result<(Value, Value, Value), Error> {
    assert_length(list, 3, fn_name)?;
    Ok((
        resolve(list.cells().get(1).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(2).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(3).unwrap().clone(), stack, fn_name)?
    ))
}

pub fn resolve_four_arguments(list: &List, stack: &mut Stack, fn_name: &'static str) -> Result<(Value, Value, Value, Value), Error> {
    assert_length(list, 4, fn_name)?;
    Ok((
        resolve(list.cells().get(1).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(2).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(3).unwrap().clone(), stack, fn_name)?,
        resolve(list.cells().get(4).unwrap().clone(), stack, fn_name)?
    ))
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

pub fn to_float(value: Value) -> Option<FLOAT> {
    match value {
        Value::Integer(i) => Some(i as FLOAT),
        Value::Float(f) => Some(f),
        _ => None
    }
}