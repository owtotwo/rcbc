use super::token::{Token, TokenKind};
use std::result;
use std::fmt;

type Result<T> = result::Result<T, ScanError>;

pub struct Scanner {
    stream: String,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum ScanError {
    // ...
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            stream: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self, stream: String) -> Result<Vec<Token>> {
        self.stream = stream;
        unimplemented!()
    }
}


impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // ...
        };
        "Scan Error".fmt(f)
    }
}