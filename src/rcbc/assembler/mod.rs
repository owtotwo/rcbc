#![allow(unused_variables, dead_code)]
use std::path::Path;

#[derive(Debug)]
pub struct Assembler;

impl Assembler {
    pub fn new() -> Assembler {
        Assembler
    }

    pub fn assemble<P: AsRef<Path>>(&self, asm_file: P, obj_file: P) -> Result<(), String> {
        unimplemented!()
    }
}