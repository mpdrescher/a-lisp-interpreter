use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub struct Error {
    message: String,
    trace: Vec<String>
}

impl Error {
    pub fn new(msg: String) -> Error {
        Error {
            message: msg,
            trace: Vec::new()
        }
    }

    pub fn add_trace(mut self, trace: String) -> Error{
        self.trace.push(trace);
        self
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { //TODO: group trace so that errors in recursion take only one line
        writeln!(f, "    Error: {}", self.message)?;
        for elem in &self.trace {
            writeln!(f, "        ...at '{}'", elem)?;
        }
        Ok(())
    }
}
