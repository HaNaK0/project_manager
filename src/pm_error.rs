use std::{error, fmt::Display};


#[derive(Debug,Clone)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Project Manager failed")
    }
}

impl error::Error for Error {}
