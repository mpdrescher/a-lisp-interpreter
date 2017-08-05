extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod interpreter;
mod value;
mod list;
mod error;
mod functions;
mod scope;
mod corelib;
mod lambda;

use interpreter::Interpreter;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                match interpreter.eval(line) {
                    Ok(v) => println!("    {}", v),
                    Err(e) => println!("Err: {:?}", e)
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("[Interrupted]");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("[EOF]");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
