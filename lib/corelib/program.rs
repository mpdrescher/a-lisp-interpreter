use list::List;
use error::Error;
use list::resolve;
use functions::assert_length;
use functions::assert_min_length;
use functions::invalid_types;
use functions::resolve_two_arguments;
use functions::resolve_argument;
use value::Value;
use lambda::Lambda;
use stack::Stack;
use interpreter::Interpreter;
use std::thread;

pub fn lambda(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "lambda")?;
    match (op_1, op_2) {
        (Value::List(params), Value::List(value)) => {
            let mut args = Vec::new();
            for param in params.cells() {
                match param {
                    &Value::Symbol(ref param_str) => {
                        args.push(param_str.clone());
                    },
                    _ => {
                        return Err(Error::new_with_origin("lambda", format!("only symbols can be used as function parameters.")));
                    }
                }
            }
            let lambda = Lambda::new(args, value);
            return Ok(Value::new_lambda(lambda));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "lambda")?;
        }
    }
    Ok(Value::Nil)
}

pub fn cond(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    assert_min_length(list, 1, "cond")?;
    for i in 1..list.cells().len() {
        let cell = resolve(list.cells().get(i).unwrap().clone(), stack, "cond")?;
        let inner_list = match cell {
            Value::List(list) => list,
            _ => {
                return Err(Error::new_with_origin("cond", format!("expected a list instead of '{}' at index {}.", cell.type_str(), i - 1)));
            }
        };
        if inner_list.cells().len() != 2 {
            return Err(Error::new_with_origin("cond", format!("expected a list with two elements at index {}, found one with {}.", i - 1, inner_list.cells().len())));
        }
        let condition_value = resolve(inner_list.cells().get(0).unwrap().clone(), stack, "cond")?;
        let condition = match condition_value {
            Value::Boolean(boolean) => boolean,
            _ => {
                return Err(Error::new_with_origin("cond", format!("expected a boolean as the first element at index {}", i - 1)));
            }
        };
        if condition {
            return Ok(resolve(inner_list.cells().get(1).unwrap().clone(), stack, "cond")?);
        }
    }
    Err(Error::new_with_origin("cond", format!("no condition was true.")))
}

pub fn set(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "set")?;
    match (op_1, op_2) {
        (type_1, Value::Symbol(_)) => {
            invalid_types(vec!(&type_1, &Value::Symbol(String::new())), "set")?;
        },
        (Value::Symbol(name), value) => {
            if stack.size() <= 1 {
                return Err(Error::new_with_origin("set", format!("no scope above the current one.")));
            }
            match stack.set_or_append_variable(name, value) {
                Ok(_) => {},
                Err(err) => {
                    return Err(err.add_trace(format!("set")));
                }
            }
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "set")?;
        }
    };
    Ok(Value::Nil)
}

pub fn global(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "global")?;
    match (op_1, op_2) {
        (type_1, Value::Symbol(_)) => {
            invalid_types(vec!(&type_1, &Value::Symbol(String::new())), "global")?;
        },
        (Value::Symbol(name), value) => {
            if stack.size() == 0 {
                return Err(Error::new_with_origin("global", format!("no scope found.")));
            }
            stack.get_mut_first().unwrap().set_variable(name, value);
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "global")?;
        }
    };
    Ok(Value::Nil)
}

pub fn quote(list: &List, _stack: &mut Stack) -> Result<Value, Error> {
    assert_length(list, 1, "quote")?;
    Ok(list.cells().get(1).unwrap().clone())
}

pub fn eval(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "eval")?;
    Ok(resolve(op_1, stack, "eval")?)
}

pub fn printfmt(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "printfmt")?;
    println!("{:?}", op_1);
    Ok(Value::Nil)
}

pub fn print(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "print")?;
    println!("{}", op_1);
    Ok(Value::Nil)
}

pub fn while_loop(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "while")?;
    match (op_1, op_2) {
        (Value::List(head), Value::List(body)) => {
            let mut last = Value::Nil;
            loop {
                match head.eval(stack, None)? {
                    Value::Boolean(boolean) => {
                        if boolean == false {
                            break;
                        }
                    },
                    type_1 => {
                        return Err(Error::new_with_origin("while", format!("expected boolean in while condition, found {}.", type_1)));
                    }
                }
                last = body.eval(stack, None)?;
            }
            return Ok(last);
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "while")?;
        }
    };
    Ok(Value::Nil)
}

pub fn spawn(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    assert_min_length(list, 2, "spawn")?;
    let mut handles = Vec::new();
    for i in 1..list.cells().len() {
        let maybe_listelem = resolve(list.cells().get(i).unwrap().clone(), stack, "spawn")?;
        let listelem = match maybe_listelem {
            Value::List(list) => list,
            type1 => {
                return Err(Error::new_with_origin("spawn", format!("thread can only evaluate a list, found {}.", type1.type_str())))
            }
        };
        let handle = thread::spawn(move || {
            let mut interpreter = Interpreter::new_empty();
            interpreter.eval(listelem)
        });
        handles.push(handle);
    }
    let mut retval = Vec::new();
    let mut thread_counter = 0;
    for elem in handles {
        retval.push(
            match elem.join() {
                Ok(maybe_value) => {
                    match maybe_value {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(err.add_trace(format!("thread {}", thread_counter)));
                        }
                    }
                },
                Err(_) => {
                    return Err(Error::new_with_origin("spawn", format!("thread {} paniced.", thread_counter)));
                }
            }
        );
        thread_counter += 1;
    }
    Ok(Value::List(List::from_cells(retval)))
}

pub fn puts(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "puts")?;
    match op_1 {
        Value::List(list) => {
            for elem in list.into_cells() {
                match elem {
                    Value::Char(ch) => {
                        print!("{}", ch);
                    },
                    _ => {
                        return Err(Error::new_with_origin("puts", format!("found non-chararacter element in string.")));
                    }
                }
            }
        },
        type_1 => {
            invalid_types(vec!(&type_1), "puts")?;
        }
    }
    Ok(Value::Nil)
}

pub fn putsln(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let result = puts(list, stack);
    println!();
    result
}

pub fn seq(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    assert_min_length(list, 2, "seq")?;
    let mut retval = Value::Nil;
    for i in 1..list.cells().len() {
        retval = resolve(list.cells().get(i).unwrap().clone(), stack, "seq")?;
    }
    Ok(retval)
}

pub fn throw(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "throw")?;
    if !op_1.is_list_and_string() {
        return Err(Error::new_with_origin("throw", format!("argument can only be a string (a list that only contains characters).")));
    }
    match op_1 {
        Value::List(list) => {
            let mut string = String::new();
            for elem in list.into_cells() {
                match elem {
                    Value::Char(ch) => {
                        string.push(ch);
                    },
                    _ => {
                        unreachable!();
                    }
                }
            }
            return Err(Error::new(string));
        },
        type_1 => {
            invalid_types(vec!(&type_1), "throw")?;
        }
    }
    Ok(Value::Nil)
}

pub fn try(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    assert_length(list, 2, "try")?;
    let op_1 = list.cells().get(1).unwrap().clone();
    let op_2 = list.cells().get(2).unwrap().clone();
    match resolve(op_1, stack, "try") {
        Ok(val) => return Ok(val),
        Err(_) => {
            return resolve(op_2, stack, "try");
        }
    }
}

pub fn try_rename(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    assert_length(list, 2, "try_rename")?;
    let op_1 = list.cells().get(1).unwrap().clone();
    let op_2 = list.cells().get(2).unwrap().clone();
    match resolve(op_1, stack, "try_rename") {
        Ok(val) => Ok(val),
        Err(mut err) => {
            match resolve(op_2, stack, "try_rename") {
                Ok(name) => {
                    match name {
                        Value::Symbol(symbol) => {
                            err.clear_trace();
                            Err(err.set_origin(symbol))
                        },
                        type_1 => {
                            Err(Error::new_with_origin("try_rename", format!("expected symbol as new trace root, found {}.", type_1.type_str())))
                        }
                    }
                },
                Err(err) => {
                    Err(err.add_trace(format!("try_rename")))
                }
            }
        }
    }
}

pub fn type_fn(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "typeof")?;
    Ok(op_1.type_value())
}

pub fn format(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    assert_min_length(list, 2, "format")?;
    let mut cell_iter = list.clone().into_cells().into_iter().skip(1);
    let mut string = String::new();
    match resolve(cell_iter.next().unwrap(), stack, "format")? {
        Value::List(inner_list) => {
            for elem in inner_list.into_cells() {
                match elem {
                    Value::Char(ch) => string.push(ch),
                    type_1 => {
                        println!("{}", type_1);
                        return Err(Error::new_with_origin("format", format!("a string only contains characters, found type {} in list.", type_1.type_str())))
                    }
                }
            }
        }
        type_1 => {
            println!("{}", type_1);
            invalid_types(vec!(&type_1), "format")?;
        }
    }
    let mut args = Vec::new();
    for cell in cell_iter {
        args.push(resolve(cell, stack, "format")?);
    }
    let mut temp = Vec::new();
    for split in string.split("$$") {
        temp.push(split);
    }
    let mut result = Vec::new();
    let templen = temp.len();
    if templen-1 != args.len() {
        return Err(Error::new_with_origin("format", format!("template and argument count does not match. template count: {}, arg count: {}", templen-1, args.len())))
    }
    let mut temp_iter = temp.into_iter();
    for i in 0..templen-1 {
        let mut current = temp_iter.next().unwrap().to_owned();
        for ch in current.chars() {
            result.push(Value::Char(ch));
        }
        current = format!("{}", args.get(i).unwrap());
        for ch in current.chars() {
            result.push(Value::Char(ch));
        }
    }
    Ok(Value::List(List::from_cells(result)))
}