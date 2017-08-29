use std::fmt::{
    Display,
    Debug
};
use std::fmt::Result as FmtResult;
use std::fmt::Formatter;

use error::Error;
use list::List;
use lambda::Lambda;

const NUMBER_CHARS: [char; 14] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '+', '-', 'e'];

#[derive(Clone, PartialEq)]
pub enum Value {
    Nil,
    List(List),
    Float(f32),
    Integer(i32),
    Symbol(String),
    Lambda(Lambda),
    Boolean(bool),
    Char(char)
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

    pub fn char_from_string(string: &str) -> Result<Value, Error> {
        if string.starts_with('\\') {
            let ch = match string {
                "\\n" => '\n',
                "\\t" => '\t',
                "\\" => '\\',
                "\\r" => '\r',
                "\\´" => '´',
                escape => {
                    return Err(Error::new(format!("unknown character escape: {}.", escape)));
                }
            };
            return Ok(Value::Char(ch));
        }
        else {
            return Ok(Value::Char(string.chars().next().expect("empty string")));
        }
    }

    pub fn new_list(list: List) -> Value {
        Value::List(list)
    }

    pub fn new_lambda(lambda: Lambda) -> Value {
        Value::Lambda(lambda)
    }
    
    pub fn is_list_and_string(&self) -> bool {
        match self {
            &Value::List(ref list) => {
                let mut result = true;
                for elem in list.cells() {
                    match elem {
                        &Value::Char(_) => {},
                        _ => {
                            result = false;
                        }
                    }
                }
                result
            },
            _ => false
        }
    }

    pub fn type_str(&self) -> &'static str {
        match *self {
            Value::Nil => "nil",
            Value::List(_) => "list",
            Value::Float(_) => "float",
            Value::Integer(_) => "integer",
            Value::Symbol(_) => "symbol",
            Value::Lambda(_) => "lambda",
            Value::Boolean(_) => "boolean",
            Value::Char(_) => "char"
        }
    }
}

impl Debug for Value {
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
                    for i in 0..count - 1 {
                        buffer.push_str(&format!("{:?}, ", list.cells().get(i).unwrap()));
                    }
                    buffer.push_str(&format!("{:?}}} [list]", list.cells().get(count-1).unwrap()));
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
            },
            &Value::Char(ref ch) => {
                write!(f, "{} [char]", ch)
            }
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
                    format!("{{}}")
                }
                else {
                    let mut buffer = String::new();
                    buffer.push('{');
                    for i in 0..count - 1 {
                        buffer.push_str(&format!("{}, ", list.cells().get(i).unwrap()));
                    }
                    buffer.push_str(&format!("{}}}", list.cells().get(count-1).unwrap()));
                    buffer
                };
                write!(f, "{}", result)
            },
            &Value::Float(ref float) => {
                write!(f, "{}", float)
            },
            &Value::Integer(ref int) => {
                write!(f, "{}", int)
            },
            &Value::Symbol(ref symbol) => {
                write!(f, "{}", symbol)
            },
            &Value::Lambda(_) => {
                write!(f, "[lambda]")
            },
            &Value::Boolean(ref boolean) => {
                write!(f, "{}", boolean)
            },
            &Value::Char(ref ch) => {
                write!(f, "{}", ch)
            }
        }
    }
}

fn is_numeric(string: &String) -> bool {
    string.chars().filter(|x| !NUMBER_CHARS.contains(x)).count() == 0
}
