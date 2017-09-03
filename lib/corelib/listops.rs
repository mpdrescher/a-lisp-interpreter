use std::collections::BTreeSet;

use ::INT;
use list::List;
use error::Error;
use functions::invalid_types;
use functions::resolve_argument;
use functions::resolve_two_arguments;
use functions::resolve_three_arguments;
use value::Value;
use stack::Stack;

pub fn last(list: &List, stack: &mut Stack) -> Result<Value, Error> {
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

pub fn init(list: &List, stack: &mut Stack) -> Result<Value, Error> {
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

pub fn tail(list: &List, stack: &mut Stack) -> Result<Value, Error> {
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

pub fn len(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "len")?;
    match op_1 {
        Value::List(list) => {
            return Ok(Value::Integer(list.cells().len() as INT));
        },
        type_1 => {
            invalid_types(vec!(&type_1), "len")?;
        }
    }
    Ok(Value::Nil)
}

pub fn nth(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "nth")?;
    match (op_1, op_2) {
        (Value::Integer(index), Value::List(list)) => {
            if index < 0 {
                return Err(Error::new_with_origin("nth", format!("index must be non-negative.")));
            }
            else if list.cells().len() == 0 || index >= list.cells().len() as INT { //TODO: rethink this
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

pub fn cons(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "cons")?;
    match (op_1, op_2) {
        (val, Value::List(mut list)) => {
            list.cells_mut().insert(0, val);
            return Ok(Value::List(list));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "cons")?;
        }
    }
    Ok(Value::Nil)
}

pub fn append(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "append")?;
    match (op_1, op_2) {
        (Value::List(list_1), Value::List(list_2)) => {
            let mut cells = list_1.into_cells();
            cells.append(&mut list_2.into_cells());
            return Ok(Value::List(List::from_cells(cells)));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "append")?;
        }
    }
    Ok(Value::Nil)
}

pub fn unique(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "unique")?;
    match op_1 {
        Value::List(list) => {
            let mut new_list = Vec::new();
            for elem in list.into_cells() {
                if !new_list.contains(&elem) {
                    new_list.push(elem);
                }
            }
            return Ok(Value::List(List::from_cells(new_list)));
        },
        type_1 => {
            invalid_types(vec!(&type_1), "unique")?;            
        }
    }
    Ok(Value::Nil)
}

pub fn map(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "map")?;
    match (op_1, op_2) {
        (Value::Lambda(mut lambda), Value::List(list)) => {
            let mut result = Vec::new();
            for value in list.into_cells().into_iter() {
                result.push(lambda.eval_with_trace(vec!(value), stack, format!("map"))?);
            }
            return Ok(Value::List(List::from_cells(result)));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "map")?;
        }
    }
    Ok(Value::Nil)
}

pub fn fold(list: &List, stack: &mut Stack) -> Result<Value, Error> {
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

pub fn expand(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2, op_3) = resolve_three_arguments(list, stack, "expand")?;
    match (op_1, op_2, op_3) {
        (first, Value::Lambda(mut lambda), Value::List(list)) => {
            let mut acc = Vec::new();
            if list.cells().len() == 0 {
                return Ok(Value::Nil);
            }
            acc.push(first);
            for elem in list.into_cells().into_iter() {
                let current = lambda.eval_with_trace(vec!(acc.last().unwrap().clone(), elem), stack, format!("expand"))?;
                acc.push(current);
            }
            acc.remove(0);
            return Ok(Value::List(List::from_cells(acc)));
        },
        (type_1, type_2, type_3) => {
            invalid_types(vec!(&type_1, &type_2, &type_3), "expand")?;
        }
    }
    Ok(Value::Nil)
}

pub fn any(list: &List, stack: &mut Stack) -> Result<Value, Error> {
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

pub fn all(list: &List, stack: &mut Stack) -> Result<Value, Error> {
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

pub fn filter(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "filter")?;
    match (op_1, op_2) {
        (Value::Lambda(mut lambda), Value::List(list)) => {
            let mut new_list = Vec::new();
            let mut index_counter = 0;
            for value in list.into_cells().into_iter() {
                let result = lambda.eval_with_trace(vec!(value.clone()), stack, format!("filter"))?;
                match result {
                    Value::Boolean(true) => {
                        new_list.push(value);
                    },
                    Value::Boolean(false) => {},
                    _ => {
                        return Err(Error::new_with_origin("filter", format!("expected boolean at index {}.", index_counter)))
                    }
                }
                index_counter += 1;
            }
            return Ok(Value::List(List::from_cells(new_list)));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "filter")?;
        }
    }
    Ok(Value::Nil)
}

pub fn find(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "find")?;
    match (op_1, op_2) {
        (value, Value::List(list)) => {
            let mut counter = 0;
            for elem in list.into_cells() {
                if elem == value {
                    return Ok(Value::Integer(counter));
                }
                counter += 1;
            }
            return Ok(Value::Integer(-1));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "find")?;
        }
    }
    Ok(Value::Nil)
}

pub fn split_at(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "split_at")?;
    match (op_1, op_2) {
        (Value::Integer(int_32), Value::List(list)) => {
            if 0 <= int_32 && int_32 <= (list.cells().len() - 1) as INT {
                let int = int_32 as usize;
                let cells = list.into_cells();
                let (l1, l2) = cells.split_at(int);
                let new_list = vec!(Value::List(List::from_cells(l1.to_owned())), Value::List(List::from_cells(l2.to_owned())));
                return Ok(Value::List(List::from_cells(new_list)));
            }
            return Err(Error::new_with_origin("split_at", format!("index out of bounds: index is {}, list size is {}.", int_32, list.cells().len())));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "split_at")?;
        }
    }
    Ok(Value::Nil)
}

pub fn combine(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "combine")?;
    match (op_1, op_2) {
        (Value::List(list_1), Value::List(list_2)) => {
            let cells_1 = list_1.into_cells();
            let cells_2 = list_2.into_cells();
            let mut result = Vec::new();
            for elem_1 in cells_1 {
                for elem_2 in cells_2.clone() {
                    result.push(Value::List(List::from_cells(vec!(elem_1.clone(), elem_2))));
                }
            }
            return Ok(Value::List(List::from_cells(result)));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "combine")?;
        }
    }
    Ok(Value::Nil)
}

pub fn intersect(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "intersect")?;
    match (op_1, op_2) {
        (Value::List(list_1), Value::List(list_2)) => {
            let cells_1 = list_1.into_cells();
            let cells_2 = list_2.into_cells();
            let mut result = Vec::new();
            for elem_1 in cells_1 {
                for elem_2 in cells_2.clone() {
                    if elem_1 == elem_2 {
                        result.push(elem_2);
                    }
                }
            }
            return Ok(Value::List(List::from_cells(result)));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "intersect")?;
        }
    }
    Ok(Value::Nil)
}

pub fn zip(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let (op_1, op_2) = resolve_two_arguments(list, stack, "zip")?;
    match (op_1, op_2) {
        (Value::List(list_1), Value::List(list_2)) => {
            let cells_1 = list_1.into_cells();
            let cells_2 = list_2.into_cells();
            let mut result = Vec::new();
            let mut counter = 0;
            for elem_1 in cells_1 {
                let elem_2 = cells_2.get(counter).unwrap_or(&Value::Nil).to_owned();
                    result.push(Value::List(List::from_cells(vec!(elem_1, elem_2))));
                counter += 1;
            }
            return Ok(Value::List(List::from_cells(result)));
        },
        (type_1, type_2) => {
            invalid_types(vec!(&type_1, &type_2), "zip")?;
        }
    }
    Ok(Value::Nil)
}

pub fn rev(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "rev")?;
    match op_1 {
        Value::List(list) => {
            return Ok(Value::List(List::from_cells(list.into_cells().into_iter().rev().collect::<Vec<Value>>())));
        },
        type_1 => {
            invalid_types(vec!(&type_1), "rev")?;
        }
    }
    Ok(Value::Nil)
}

// TODO: f32 doesnt implement Ord, so I have to find another way to sort lists consisting of ints/floats
pub fn sort(list: &List, stack: &mut Stack) -> Result<Value, Error> {
    let op_1 = resolve_argument(list, stack, "sort")?;
    match op_1 {
        Value::List(list) => {
            let mut sorted = BTreeSet::new();
            let cells = list.into_cells();
            for elem in cells.into_iter() {
                match elem {
                    Value::Integer(int) => {
                        sorted.insert(int);
                    },
                    type_2 => {
                        return Err(Error::new_with_origin("sort", format!("only lists that contain integers can be sorted. found {}.", type_2.type_str())))
                    }
                }
            }
            let new_cells = sorted.into_iter()
                .map(|x| Value::Integer(x))
                .collect::<Vec<Value>>();
            return Ok(Value::List(List::from_cells(new_cells)));
        },
        type_1 => {
            invalid_types(vec!(&type_1), "sort")?;
        }
    }
    Ok(Value::Nil)
}

//TODO: add function 'shape' which gives the nested size of a nested list like apls shape
//TODO: add function 'split'