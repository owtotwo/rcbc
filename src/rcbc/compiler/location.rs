use std::path::Path;

#[derive(Debug, Clone)]
pub struct Location<'a> {
    file: &'a Path,
    begin_line: usize,
    begin_column: usize,
    end_line: usize,
    end_column: usize,
}

impl<'a> Location<'a> {
    pub fn new(file: &'a Path, begin_line: usize,
               begin_column: usize, end_line: usize, 
               end_column: usize) -> Location<'a> {
        Location {
            file: file,
            begin_line: begin_line,
            begin_column: begin_column,
            end_line: end_line,
            end_column: end_column,
        }
    }

    pub fn line(&self) -> usize {
        self.begin_line()
    }

    pub fn column(&self) -> usize {
        self.begin_column()
    }

    pub fn begin_line(&self) -> usize {
        self.begin_line
    }

    pub fn begin_column(&self) -> usize {
        self.begin_column
    }

    pub fn end_line(&self) -> usize {
        self.end_line
    }

    pub fn end_column(&self) -> usize {
        self.end_column
    }
    
}