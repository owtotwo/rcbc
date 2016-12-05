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
                Some(ref c) if c.is_alphabetic() || *c == '_' => 
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
            Some(pos) => { self.step(pos); },
            None => { self.iter.by_ref().count(); } // eat all the chars
        };

        // self.tokens.push(Token::new(TokenKind::Space, None));

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
        let s = self.iter.as_str();
    
        macro_rules! match_keyword {
            ($Kw_str: expr, $Kw_kind: ident) => (
                if s.starts_with($Kw_str) {
                    match s.chars().nth($Kw_str.len()) {
                        c @ None | c @ Some(_) if c.is_none() || 
                                !c.unwrap().is_alphanumeric() &&
                                 c.unwrap() != '_' => {
                            self.tokens.push(Token::new(
                                TokenKind::$Kw_kind, None));
                            self.step($Kw_str.len());
                            return Ok(());
                        }
                        _ => return self.scan_identifier(),
                    }
                }
            );
        };

        match_keyword!("void", Void);
        match_keyword!("char", Char);
        match_keyword!("short", Short);
        match_keyword!("int", Int);
        match_keyword!("long", Long);
        match_keyword!("struct", Struct);
        match_keyword!("union", Union);
        match_keyword!("enum", Enum);
        match_keyword!("static", Static);
        match_keyword!("extern", Extern);
        match_keyword!("const", Const);
        match_keyword!("signed", Signed);
        match_keyword!("unsigned", Unsigned);
        match_keyword!("if", If);
        match_keyword!("else", Else);
        match_keyword!("switch", Switch);
        match_keyword!("case", Case);
        match_keyword!("default", Default);
        match_keyword!("while", While);
        match_keyword!("do", Do);
        match_keyword!("for", For);
        match_keyword!("return", Return);
        match_keyword!("break", Break);
        match_keyword!("continue", Continue);
        match_keyword!("goto", Goto);
        match_keyword!("typedef", Typedef);
        match_keyword!("import", Import);
        match_keyword!("sizeof", Sizeof);

        self.scan_identifier()
    }

    fn scan_identifier(&mut self) -> Result<()> {
        // eat the first char of identifier
        let c = self.iter.clone().next().unwrap();
        assert!(c.is_alphabetic() || c == '_');

        let mut scout = self.iter.clone();

        match scout.position(|c| !c.is_alphanumeric() && c != '_') {
            Some(pos) => {
                let identifier = self.step(pos);
                self.tokens.push(Token::new(
                    TokenKind::Identifier, Some(identifier)));
            },
            None => { // EOF
                let identifier = self.iter.as_str().to_string();
                self.iter.by_ref().count(); // eat all
                self.tokens.push(Token::new(
                    TokenKind::Identifier, Some(identifier)));
            }
        }

        Ok(())
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

    fn step(&mut self, n: usize) -> String {
        let mut content = String::new();
        for _ in 0..n {
            match self.iter.next() {
                Some(ref c) => {
                    content.push(*c);
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
        content
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