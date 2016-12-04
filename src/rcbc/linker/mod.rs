#![allow(unused_variables, dead_code)]
use std::path::Path;

#[derive(Debug)]
pub struct Linker;

impl Linker {
    pub fn new() -> Linker {
        Linker
    }

    pub fn link<P: AsRef<Path>>(&self, obj_files: &Vec<P>, exec_file: &P) -> Result<(), String> {
        unimplemented!()
    }
}