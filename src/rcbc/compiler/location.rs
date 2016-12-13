#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub begin: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(begin_line: usize, begin_column: usize,
               end_line: usize, end_column: usize) -> Location {
        Location {
            begin: Position::new(begin_line, begin_column),
            end: Position::new(end_line, end_column),
        }
    }
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position { line: line, column: column }
    }
}