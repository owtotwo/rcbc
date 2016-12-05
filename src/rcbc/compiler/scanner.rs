#![allow(unused_imports, unreachable_code)]
use super::token::{Token, TokenKind};
use std::result;
use std::fmt;
use std::str::Chars;
use std::iter::Peekable;

type Result<T> = result::Result<T, ScanError>;

pub struct Scanner<'a> {
    iter: Chars<'a>,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ScanError {
    line: usize,
    column: usize,
    kind: ScanErrorKind,
}

#[derive(Debug)]
pub enum ScanErrorKind {
    CommentBlockNotClosing,
}

impl<'a> Scanner<'a> {
    pub fn new(stream: &'a String) -> Scanner<'a> {
        Scanner {
            iter: stream.chars(),
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>> {
        // let mut tokens: Vec<Token> = Vec::new();
        self.lexical_analysis() ?;
        Ok(self.tokens.clone())
    }

    fn lexical_analysis(&mut self) -> Result<()> {
        // lookahead
        loop {
            let mut scout = self.iter.clone();
            match scout.next() {
                Some(ref c) if c.is_whitespace() =>
                    self.scan_space() ?,
                Some('/') => 
                    match scout.next() {
                        Some('*') => self.scan_block_comment() ?,
                        Some('/') => self.scan_line_comment() ?,
                        _         => self.scan_others() ?,
                    },
                Some('\'') =>
                    self.scan_character_literal() ?,
                Some('\"') =>
                    self.scan_string_literal() ?,
                Some(ref c) if c.is_lowercase() =>
                    self.scan_reserved_words_or_identifier() ?,
                Some(ref c) if c.is_alphanumeric() || *c == '_' => 
                    self.scan_identifier() ?,
                Some(ref c) if c.is_digit(10) =>
                    self.scan_integer() ?,
                None => break,
                _ => unreachable!(),
            };
        }
        Ok(())
    }

    fn scan_space(&mut self) -> Result<()> {
        let mut scout = self.iter.clone();

        match scout.position(|c| !c.is_whitespace()) {
            Some(0) => panic!("Ln {}, Col {}", self.line, self.column),
            // None if all is space.
            Some(pos) => self.step(pos),
            None => { self.iter.by_ref().count(); } // eat all the chars
        };

        self.tokens.push(Token::new(TokenKind::Space, None));

        Ok(())
    }

    fn scan_block_comment(&mut self) -> Result<()> {
        assert!(self.iter.as_str().starts_with("/*"));
        self.step(2);

        // find "*/"
        let mut scout = self.iter.clone();
        let mut move_count = 0;
        loop {
            match scout.by_ref().position(|c| c == '*') {
                Some(pos) => {
                    move_count += pos + 1;
                    if scout.clone().next() == Some('/') {
                        self.step(move_count + 1);
                        return Ok(())
                    }
                },
                None => return Err(ScanError::new(self.line, self.column,
                    ScanErrorKind::CommentBlockNotClosing)),
            };
        }
    }

    fn scan_line_comment(&mut self) -> Result<()> {
        assert!(self.iter.as_str().starts_with("//"));
        self.step(2);

        // find '\n'
        let mut scout = self.iter.clone();

        if let Some(pos) = scout.position(|c| c == '\n') {
            self.step(pos + 1);
        }
        Ok(())
    }

    fn scan_reserved_words_or_identifier(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn scan_identifier(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn scan_integer(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn scan_character_literal(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn scan_string_literal(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn scan_others(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n {
            match self.iter.next() {
                Some(ref c) => {
                    if *c == '\n' {
                        self.line += 1;
                        self.column = 0;
                    } else {
                        self.column += 1;
                    }
                }
                _ => unreachable!()
            }
        }
    }
}


impl ScanError {
    fn new(line: usize, column: usize, kind: ScanErrorKind) -> ScanError {
        ScanError { line: line, column: column, kind: kind }
    }
}


impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}: ", self.line, self.column) ?;
        match self.kind {
            ScanErrorKind::CommentBlockNotClosing =>
                write!(f, "the comment block is not closing"),
        }
    }
}