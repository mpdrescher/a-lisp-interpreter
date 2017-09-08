extern crate sdl2;

pub mod corelib;
pub mod error;
pub mod functions;
pub mod interpreter;
pub mod lambda;
pub mod list;
pub mod scope;
pub mod value;
pub mod stack;

pub type FLOAT = f64;
pub type INT = i64;