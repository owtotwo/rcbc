#![allow(unused_imports, unreachable_code)]
use super::token::{Token, TokenKind};
use std::result;
use std::fmt;
use std::str::Chars;

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
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = stream.chars();
        Scanner::lexical_analysis(&mut iter, &mut tokens) ?;
        Ok(tokens)
    }

    fn lexical_analysis(iter: &mut Chars, tokens: &mut Vec<Token>)
            -> Result<()> {
        let mut scout = iter.clone();
        // lookahead
        match scout.next() {
            Some(' ') | Some('\t') | Some('\n') | Some('\r') =>
                Scanner::scan_space(iter, tokens),
            Some('/') => 
                match scout.next() {
                    Some('*') => Scanner::scan_block_comment(iter, tokens),
                    Some('/') => Scanner::scan_line_comment (iter, tokens),
                    _         => Scanner::scan_others       (iter, tokens),
                },
            Some('\'') =>
                Scanner::scan_character_literal(iter, tokens),
            Some('\"') =>
                Scanner::scan_string_literal(iter, tokens),
            Some(ref c) if c.is_lowercase() =>
                Scanner::scan_reserved_words_or_identifier(iter, tokens),
            Some(ref c) if c.is_alphanumeric() || c == '_' => 
                Scanner::scan_identifier(iter, tokens),
            Some(ref c) if c.is_digit() =>
                Scanner::scan_integer(iter, tokens),
            None => unimplemented!(),
            _ => unimplemented!(),
        }
        
    }

    fn scan_space(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
        unimplemented!()
    }

    fn scan_block_comment(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
        unimplemented!()
    }

    fn scan_line_comment(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
        unimplemented!()
    }

    fn scan_reserved_words_or_identifier(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
        unimplemented!()
    }

    fn scan_identifier(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
        unimplemented!()
    }

    fn scan_integer(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
        unimplemented!()
    }

    fn scan_character_literal(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
        unimplemented!()
    }

    fn scan_string_literal(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
        unimplemented!()
    }

    fn scan_others(iter: &mut Chars, tokens: &mut Vec<Token>) -> Result<()> {
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