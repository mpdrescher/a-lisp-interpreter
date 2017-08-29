extern crate rustyline;
extern crate alisplib;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use alisplib::interpreter::Interpreter;

use std::env;
use std::io::Result as IOResult;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() == 0 {
        start_interactive();
    }
    else {
        for file in args {
            match start_script(file) {
                Ok(_) => {},
                Err(e) => {
                    println!("file error: {}", e);
                }
            }
        }
    }
}

fn start_script(path: String) -> IOResult<()> {
    let mut interpreter = Interpreter::new();
    interpreter.load_script(path)
}

//TODO: ignore strings when deciding between ... and >>>

fn start_interactive() {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::new();
    println!("== A Lisp Interpreter ==");
    println!("-- under construction --");
    println!();
    loop {
        let (line, quit) = unwrap_readline(rl.readline(">>> "));
        if quit {
            return;
        }
        if unclosed_brackets(&line) == 0 {
            rl.add_history_entry(&line);
            eval(&mut interpreter, line);
        }
        else {
            let mut buffer = line;
            buffer.push(' ');
            while unclosed_brackets(&buffer) != 0 {
                let (inner_line, quit) = unwrap_readline(rl.readline("... "));
                if quit {
                    return;
                }
                buffer.push_str(&inner_line);
                buffer.push(' ');
            }
            rl.add_history_entry(&buffer);
            eval(&mut interpreter, buffer);
        }
    }
}

fn eval(interpreter: &mut Interpreter, code: String) {
    println!();
    match interpreter.eval_string(code) {
        Ok(v) => println!("    {}\n", v),
        Err(e) => println!("{}", e)
    }
}

//the bool determines wether to quit or not
fn unwrap_readline(line: Result<String, ReadlineError>) -> (String, bool) {
    let empty = String::new();
    match line {
        Ok(line) => {
            (line, false)
        },
        Err(ReadlineError::Interrupted) => {
            println!("\n    [Interrupted]\n");
            (empty, true)
        },
        Err(ReadlineError::Eof) => {
            println!("\n    [EOF]\n");
            (empty, true)
        },
        Err(err) => {
            println!("\n    Error: {:?}\n", err);
            (empty, true)
        }
    }
}

fn unclosed_brackets(string: &String) -> isize {
    let mut acc = 0;
    let mut quoted = false;
    for ch in string.chars() {
        match ch {
            '"' => {
                quoted = !quoted;
            },
            '(' => {
                if quoted {
                    continue;
                }
                acc += 1;
            },
            ')' => {
                if quoted {
                    continue;
                }
                acc -= 1;
            },
            _ => {}
        }
    }
    acc
}