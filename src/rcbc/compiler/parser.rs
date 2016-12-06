#![allow(unreachable_code, unused_imports)]
use super::token::{Token, TokenKind};
use std::result;
use std::fmt;
use std::slice::Iter;

type Result<T> = result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    iter: Iter<'a, Token>,
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseErrorKind {
    // ...
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            iter: token_stream.iter(),
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        unimplemented!()
    }
}


impl ParseError {
    fn new(line: usize, column: usize, kind: ParseErrorKind) -> ParseError {
        ParseError {
            kind: kind,
            line: line,
            column: column,
        }
    }
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ln {}, Col {}: ", self.line, self.column) ?;
        match self.kind {
            
        }
    }
}