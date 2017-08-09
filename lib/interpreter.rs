use value::Value;
use error::Error;
use list::List;
use scope::Scope;

use std::env;
use std::io::Read;
use std::io::Result as IOResult;
use std::fs::File;

pub struct Interpreter {
    global: Scope
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            global: Scope::new()
        }
    }

    pub fn eval(&mut self, code: String) -> Result<Value, Error> {
        let mut list = List::from_string(code)?;
        let mut stack = vec!(self.global.clone());
        let result = list.eval(&mut stack, None);
        self.global = stack.into_iter().next().unwrap();
        result
    }

    pub fn load_script(&mut self, path: String) -> IOResult<()> {
        let mut code = String::new();
        let mut file = File::open(path)?;
        let _ = file.read_to_string(&mut code)?;
        let mut interpreter = Interpreter::new();
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
                                    match self.eval(line_buffer) {
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
