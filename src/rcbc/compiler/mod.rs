//! The main class for the compiler, which is in a narrow sense.
//!
//! Do scanning, parsing, IR generation, optimization, assemblies generation.
//! The assembler and link do not be included.
//!
//! Have fun.

use std::path::Path;

const COMPILER_NAME:    &'static str = "rcbc";
const COMPILER_VERSION: &'static str = "0.0.1";

#[derive(Debug)]
pub struct Compiler {
    name: &'static str,
    version: &'static str,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            name: COMPILER_NAME,
            version: COMPILER_VERSION,
        }
    }

    pub fn compile<P: AsRef<Path>>(&self, src_file: P, asm_file: P) -> Result<(), String> {
        unimplemented!()
    }
}