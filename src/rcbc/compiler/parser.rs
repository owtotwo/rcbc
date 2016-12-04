#![allow(unreachable_code, unused_imports)]
use super::token::{Token, TokenKind};
use std::result;
use std::fmt;

type Result<T> = result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct Parser;

#[derive(Debug)]
pub enum ParseError {
    // ...
}

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    pub fn parse(&mut self, token_stream: Vec<Token>) -> Result<()> {
        unimplemented!()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // ...
        };
        "Parse Error".fmt(f)
    }
}