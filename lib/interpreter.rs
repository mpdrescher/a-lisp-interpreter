use value::Value;
use error::Error;
use list::List;
use scope::Scope;

use std::io::Read;
use std::io::Result as IOResult;
use std::fs::File;
use stack::Stack;

pub const STD_LIST: [&'static str; 3] = [
    "std/basics.ali",
    "std/assert.ali",
    "std/simplemath.ali"
];

//TODO: change printlns to function return values

pub struct Interpreter {
    global: Scope
}

impl Interpreter {
    pub fn load_std(&mut self) -> IOResult<()> {
        for elem in STD_LIST.iter() {
            self.load_script(format!("{}", elem))?;
        }
        Ok(())
    }

    pub fn new_empty() -> Interpreter {
        Interpreter {
            global: Scope::new()
        }
    }

    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter::new_empty();
        match interpreter.load_std() {
            Ok(_) => {},
            Err(err) => {
                println!("error while loading std:");
                println!("{}", err);
            }
        }
        interpreter
    }

    pub fn eval_string(&mut self, code: String) -> Result<Value, Error> {
        let list = List::from_string(code)?;
        self.eval(list)
    }

    pub fn eval(&mut self, list: List) -> Result<Value, Error> {
        let mut stack = Stack::from_scopes(vec!(self.global.clone()));
        let result = list.eval(&mut stack, None);
        self.global = stack.into_first_scope().unwrap();
        result
    }

    pub fn load_script(&mut self, path: String) -> IOResult<()> {
        let mut code = String::new();
        let mut file = File::open(path)?;
        let _ = file.read_to_string(&mut code)?;
        let mut line_buffer = String::new();
        let mut code_iter = code.chars();
        loop {
            match code_iter.next() {
                Some('(') => {
                    let mut bracket_balance = 0; //TODO: make this a function and replace it in lib::list
                    loop {
                        match code_iter.next() {
                            Some('(') => {
                                bracket_balance += 1;
                                line_buffer.push('(');
                            },
                            Some(')') => {
                                if bracket_balance == 0 {
                                    match self.eval_string(line_buffer) {
                                        Ok(_) => {},
                                        Err(e) => println!("{}", e)
                                    }
                                    line_buffer = String::new();
                                    break;
                                }
                                else {
                                    line_buffer.push(')');
                                }
                                bracket_balance -= 1;
                            },
                            Some(ch) => {
                                line_buffer.push(ch);
                            }
                            None => {
                                println!("reached end of file before closing bracket");
                                break;
                            }
                        };
                    }
                },
                Some(_) => {},
                None => {
                    break;
                }
            }
        }
        Ok(())
    }
}