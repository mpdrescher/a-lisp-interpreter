use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::fmt::Formatter;

use list::List;
use lambda::Lambda;

const NUMBER_CHARS: [char; 14] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '+', '-', 'e'];

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    List(List),
    Float(f32),
    Integer(i32),
    Symbol(String),
    Lambda(Lambda),
    Boolean(bool)
}

impl Value {
    pub fn from_string(code: String) -> Value {
        if code == "nil" {
            Value::Nil
        }
        else if code == "true" {
            Value::Boolean(true)
        }
        else if code == "false" {
            Value::Boolean(false)
        }
        else if is_numeric(&code) {
            let dot_count = code.chars().filter(|x| *x == '.').count();
            if dot_count == 1 {
                match code.parse::<f32>() {
                    Ok(float) => {
                        Value::Float(float)
                    },
                    Err(_) => {
                        Value::Symbol(code)
                    }
                }
            }
            else if dot_count == 0 {
                match code.parse::<i32>() {
                    Ok(int) => {
                        Value::Integer(int)
                    },
                    Err(_) => {
                        Value::Symbol(code)
                    }
                }
            }
            else {
                Value::Symbol(code)
            }
        }
        else {
            Value::Symbol(code)
        }   
    }

    pub fn new_list(list: List) -> Value {
        Value::List(list)
    }

    pub fn new_lambda(lambda: Lambda) -> Value {
        Value::Lambda(lambda)
    }

    pub fn type_str(&self) -> &'static str {
        match *self {
            Value::Nil => "nil",
            Value::List(_) => "list",
            Value::Float(_) => "float",
            Value::Integer(_) => "integer",
            Value::Symbol(_) => "symbol",
            Value::Lambda(_) => "lambda",
            Value::Boolean(_) => "boolean"
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &Value::Nil => {
                write!(f, "[nil]")
            },
            &Value::List(ref list) => {
                let count = list.cells().len();
                let result = if count == 0 {
                    format!("{{<empty>}} [list]")
                }
                else {
                    let mut buffer = String::new();
                    buffer.push('{');
                    for i in 0..count-1 {
                        buffer.push_str(&format!("{}, ", list.cells().get(i).unwrap()));
                    }
                    buffer.push_str(&format!("{}}} [list]", list.cells().get(count-1).unwrap()));
                    buffer
                };
                write!(f, "{}", result)
            },
            &Value::Float(ref float) => {
                write!(f, "{} [float]", float)
            },
            &Value::Integer(ref int) => {
                write!(f, "{} [integer]", int)
            },
            &Value::Symbol(ref symbol) => {
                write!(f, "{} [symbol]", symbol)
            },
            &Value::Lambda(_) => {
                write!(f, "[lambda]")
            },
            &Value::Boolean(ref boolean) => {
                write!(f, "{} [boolean]", boolean)
            } 
        }
    }
}

fn is_numeric(string: &String) -> bool {
    string.chars().filter(|x| !NUMBER_CHARS.contains(x)).count() == 0
}
