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
