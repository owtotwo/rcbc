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
    stray: Option<char>,
}

#[derive(Debug)]
pub enum ScanErrorKind {
    CommentBlockNotClosing,
    InvalidOctalChar,
    InvalidChar,
    NotClosingSingalquote,
    NotClosingDoublequote,
    StrayChars,
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
                    self.scan_space(),
                Some('/') => 
                    match scout.next() {
                        Some('*') => self.scan_block_comment(),
                        Some('/') => self.scan_line_comment(),
                        _         => self.scan_operator(),
                    },
                Some('\'') =>
                    self.scan_character_literal(),
                Some('\"') =>
                    self.scan_string_literal(),
                Some(ref c) if c.is_lowercase() =>
                    self.scan_reserved_words_or_identifier(),
                Some(ref c) if c.is_alphabetic() || *c == '_' => 
                    self.scan_identifier(),
                Some(ref c) if c.is_digit(10) =>
                    self.scan_integer(),
                Some(ref c) => self.scan_operator(),
                None => {
                    self.tokens.push(Token::new(
                        TokenKind::EOF, None, self.line, self.column));
                    break;
                },
            } ?;
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

        // self.tokens.push(Token::new(TokenKind::Space, None, self.line, self.column));

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
                    ScanErrorKind::CommentBlockNotClosing, None)),
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
                                TokenKind::$Kw_kind, None,
                                self.line, self.column));
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
        // ensure the first char of identifier
        let c = self.iter.clone().next().unwrap();
        assert!(c.is_alphabetic() || c == '_');

        let mut scout = self.iter.clone();

        match scout.position(|c| !c.is_alphanumeric() && c != '_') {
            Some(pos) => {
                let (ln, col) = (self.line, self.column);
                let identifier = self.step(pos);
                self.tokens.push(Token::new(
                    TokenKind::Identifier, Some(identifier), ln, col));
            },
            None => { // EOF
                let identifier = self.iter.as_str().to_string();
                self.iter.by_ref().count(); // eat all chars
                self.tokens.push(Token::new(
                    TokenKind::Identifier, Some(identifier),
                    self.line, self.column));
            }
        }

        Ok(())
    }

    fn scan_integer(&mut self) -> Result<()> {
        let mut scout = self.iter.clone();

        let mut move_count = match scout.next() {
            Some('0') => {
                if scout.next() == Some('x') || scout.next() == Some('X') {
                    self.scan_hexadecimal()
                } else {
                    self.scan_octal()
                }
            }
            Some('1' ... '9') =>
                self.scan_decimal(),
            _ => unreachable!(),
        } ?;

        let mut scout = self.iter.clone().skip(move_count).peekable();

        // suffix: 'U' or 'L'
        if scout.peek() == Some(&'U') {
            scout.next().unwrap();
            move_count += 1;
        }
        if scout.peek() == Some(&'L') {
            scout.next().unwrap();
            move_count += 1;
        }

        let (ln, col) = (self.line, self.column);
        let integer = self.step(move_count);
        self.tokens.push(Token::new(TokenKind::Integer, Some(integer), ln, col));

        Ok(())
    }

    fn scan_decimal(&mut self) -> Result<usize> {
        let mut scout = self.iter.clone();
        match scout.position(|c| !c.is_digit(10)) {
            Some(0) => unreachable!(),
            Some(pos) => Ok(pos),
            None => Ok(self.iter.clone().count()),
        }
    }

    // This function could not ensure that the first char of self.iter
    // is zero (the valid head of octal number).
    fn scan_octal(&mut self) -> Result<usize> {
        let mut scout = self.iter.clone();
        match scout.position(|c| !c.is_digit(8)) {
            Some(0) => unreachable!(),
            Some(pos) => Ok(pos),
            None => Ok(self.iter.clone().count()),
        }
    }

    // This function could not ensure that the first two chars of
    // self.iter are zero and alpha 'x' or 'X' (the valid head of
    // hexadecimal number).
    fn scan_hexadecimal(&mut self) -> Result<usize> {
        let mut scout = self.iter.clone().skip(2);
        match scout.position(|c| !c.is_digit(16)) {
            Some(0) => unreachable!(),
            Some(pos) => Ok(pos + 2),
            None => Ok(self.iter.clone().count()),
        }
    }

    fn scan_character_literal(&mut self) -> Result<()> {
        let mut scout = self.iter.clone();
        assert_eq!(scout.next(), Some('\''));

        let move_count;

        match scout.next() {
            // octal represent a char
            Some('\\') => {
                match scout.next() {
                    // regex `'\\[0-7]{3}'`
                    Some('0' ... '7') => {
                        for _ in 0..2 { match scout.next() {
                            Some('0' ... '7') => { /* should be */ },
                            _ => return Err(ScanError::new(self.line,
                                self.column,
                                ScanErrorKind::InvalidOctalChar,
                                None
                            )),
                        }}
                        move_count = 6;
                    },
                    // regex `'\\.'`
                    _ => { // an arbitrary char
                        move_count = 4;
                    },
                }
            },
            Some(ref c) => {
                if *c == '\n' {
                    return Err(ScanError::new(self.line, self.column,
                        ScanErrorKind::InvalidChar, None));
                }
                move_count = 3;
            }
            None => {
                return Err(ScanError::new(self.line, self.column,
                    ScanErrorKind::NotClosingSingalquote, None));
            }
        }

        // The closing single quote
        match scout.next() {
            Some('\'') => {
                let (ln, col) = (self.line, self.column);
                let character = self.step(move_count);
                self.tokens.push(Token::new(
                    TokenKind::Character, Some(character), ln, col));
            },
            _ => return Err(ScanError::new(self.line, self.column,
                    ScanErrorKind::NotClosingSingalquote, None)),
        }

        Ok(())
    }

    fn scan_string_literal(&mut self) -> Result<()> {
        let mut scout = self.iter.clone();
        assert_eq!(scout.next(), Some('\"'));

        let mut move_count = 1;

        loop { match scout.next() {
            // end of string
            Some('\"') => {
                move_count += 1;
                let (ln, col) = (self.line, self.column);
                let string = self.step(move_count);
                self.tokens.push(Token::new(
                    TokenKind::String, Some(string), ln, col));
                return Ok(());
            },
            // escape char
            Some('\\') => {
                match scout.next() {
                    // regex `"...(\\[0-7]{3})..."`
                    Some('0' ... '7') => {
                        for _ in 0..2 { match scout.next() {
                            Some('0' ... '7') => { /* should be */ },
                            _ => return Err(ScanError::new(self.line,
                                self.column,
                                ScanErrorKind::InvalidOctalChar,
                                None
                            )),
                        }}
                        move_count += 4;
                    },
                    // regex `"...(\\.)..."`
                    _ => { // an arbitrary char, but '\n' to none
                        move_count += 2;
                    },
                }
            },
            Some(ref c) if *c != '\n' => {
                move_count += 1;
            },
            _ => {
                return Err(ScanError::new(self.line, self.column,
                    ScanErrorKind::NotClosingDoublequote, None));
            }
        }}
    }

    fn scan_operator(&mut self) -> Result<()> {
        let s = self.iter.as_str();

        macro_rules! match_operator {
            ($Kw_str: expr, $Kw_kind: ident) => (
                if s.starts_with($Kw_str) {
                    let (ln, col) = (self.line, self.column);
                    self.step($Kw_str.len());
                    self.tokens.push(Token::new(
                        TokenKind::$Kw_kind, None, ln, col));                    
                    return Ok(())
                }
            );
        };

        match_operator!("<<=", LeftShiftAssign);
        match_operator!(">>=", RightShiftAssign);

        match_operator!("==", DoubleEquals);
        match_operator!("!=", NotEqualTo);
        match_operator!("<=", LessThanOrEqualTo);
        match_operator!(">=", GreaterThanOrEqualTo);
        match_operator!("+=", AddAssign);
        match_operator!("-=", SubtractAssign);
        match_operator!("*=", MultiplyAssign);
        match_operator!("/=", DivideAssign);
        match_operator!("%=", ModuloAssign);
        match_operator!("&=", AndAssign);
        match_operator!("^=", ExclusiveOrAssign);
        match_operator!("|=", OrAssign);
        match_operator!("&&", LogicalAnd);
        match_operator!("||", LogicalOr);
        match_operator!("<<", LeftShift);
        match_operator!(">>", RightShift);
        match_operator!("++", Increment);
        match_operator!("--", Decrement);
        match_operator!("->", Arrow);

        match_operator!(",", Comma);
        match_operator!(":", Colon);
        match_operator!(";", Semicolon);
        match_operator!("=", Equals);
        match_operator!("_", Underscore);
        match_operator!("<", LessThan);
        match_operator!(">", GreaterThan);
        match_operator!("[", OpeningBracket);
        match_operator!("]", ClosingBracket);
        match_operator!("{", LeftCurlyBracket);
        match_operator!("}", RightCurlyBracket);
        match_operator!("(", OpenParentheses);
        match_operator!(")", CloseParentheses);
        match_operator!("\'", SingleQuote);
        match_operator!("\"", DoubleQuotes);
        match_operator!(".", Dot);
        match_operator!("/", Slash);
        match_operator!("\\", Backslash);
        match_operator!("+", Plus);
        match_operator!("-", Hyphen);
        match_operator!("?", QuestionMark);
        match_operator!("!", ExclamationMark);
        match_operator!("~", Tilde);
        match_operator!("#", Number);
        match_operator!("|", VerticalBar);
        match_operator!("*", Asterisk);
        match_operator!("%", Procenttecken);
        match_operator!("^", Caret);
        match_operator!("&", Ampersand);
        match_operator!("$", Dollar);

        let stray = s.chars().next().unwrap();
        Err(ScanError::new(self.line, self.column,
                           ScanErrorKind::StrayChars, Some(stray)))
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
    fn new(line: usize, column: usize, kind: ScanErrorKind, 
           stray: Option<char>) -> ScanError {
        ScanError { line: line, column: column, kind: kind, stray: stray }
    }
}


impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ln {}, Col {}: ", self.line, self.column) ?;
        match self.kind {
            ScanErrorKind::CommentBlockNotClosing =>
                write!(f, "the comment block is not closing"),
            ScanErrorKind::InvalidOctalChar =>
                write!(f, "invalid octal number represented char"),
            ScanErrorKind::InvalidChar =>
                write!(f, "invalid char"),
            ScanErrorKind::NotClosingSingalquote =>
                write!(f, "need a closing single quote for the char"),
            ScanErrorKind::NotClosingDoublequote =>
                write!(f, "need a closing double quote for a string"),
            ScanErrorKind::StrayChars =>
                write!(f, "stray ‘{}’ in program", self.stray.unwrap()),
        }
    }
}