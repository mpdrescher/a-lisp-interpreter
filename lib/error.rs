use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub struct Error {
    origin: Option<String>,
    message: String,
    trace: Vec<String>
}

impl Error {
    pub fn new(msg: String) -> Error {
        Error {
            origin: None,
            message: msg,
            trace: Vec::new()
        }
    }

    pub fn new_with_origin(origin: &'static str, msg: String) -> Error {
        Error {
            origin: Some(origin.to_owned()),
            message: msg,
            trace: Vec::new()
        }
    }

    pub fn set_origin(mut self, origin: String) -> Error {
        self.origin = Some(origin);
        self
    } 

    pub fn add_trace(mut self, trace: String) -> Error{
        self.trace.push(trace);
        self
    }

    pub fn clear_trace(&mut self) {
        self.trace.clear();
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { //TODO: group trace so that errors in recursion take only one line
        match self.origin {
            Some(ref origin) => {
                writeln!(f, "Error: '{}': {}", origin, self.message)?;
            },
            None => {
                writeln!(f, "Error: {}", self.message)?;
            }
        }
        for elem in &self.trace {
            writeln!(f, "    ...at '{}'", elem)?;
        }
        Ok(())
    }
}
