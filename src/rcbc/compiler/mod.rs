//! The main class for the compiler, which is in a narrow sense.
//!
//! Do scanning, parsing, IR generation, optimization, assemblies generation.
//! The assembler and link do not be included.
//!
//! Have fun.

use std::path::Path;
use std::io::{self, Read, Write};
use std::fs::File;
use std::fmt;
use std::result;
use self::scanner::{Scanner, ScanError};
use self::parser::{Parser, ParseError};

mod scanner;
mod token;
mod parser;

const COMPILER_NAME:    &'static str = "rcbc";
const COMPILER_VERSION: &'static str = "0.0.1";

type Result<T> = result::Result<T, CompileError>;

#[derive(Debug)]
pub struct Compiler {
    name: &'static str,
    version: &'static str,
}

#[derive(Debug)]
pub enum CompileError {
    IO(io::Error),
    Scan(ScanError),
    Parse(ParseError),
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            name: COMPILER_NAME,
            version: COMPILER_VERSION,
        }
    }

    pub fn compile(&self, src_file: &Path, asm_file: &Path) -> Result<()> {
        println!("I will compile these files: \n{}\nto\n{}", 
                 src_file.to_str().unwrap(), asm_file.to_str().unwrap());
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let mut char_stream = String::new();

        File::open(src_file)
             .and_then(|mut src| src.read_to_string(&mut char_stream)) ?;

        let token_stream = scanner.scan(char_stream) ?;

        let ast = parser.parse(token_stream) ?;

        File::create(src_file)
             .and_then(|mut asm| asm.write("Something...".as_bytes())) ?;
        Ok(())
    }
}

impl From<io::Error> for CompileError {
    fn from(err: io::Error) -> CompileError {
        CompileError::IO(err)
    }
}

impl From<ScanError> for CompileError {
    fn from(err: ScanError) -> CompileError {
        CompileError::Scan(err)
    }
}

impl From<ParseError> for CompileError {
    fn from(err: ParseError) -> CompileError {
        CompileError::Parse(err)
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompileError::IO(ref err) =>
                write!(f, "Compiler Error: {}", err),
            CompileError::Scan(ref err) =>
                write!(f, "Compiler Error: {}", err),
            CompileError::Parse(ref err) =>
                write!(f, "Compiler Error: {}", err),
        }
    }
}