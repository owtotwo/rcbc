use std::fmt;

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
    pub fn new(begin_line: usize,
               begin_column: usize,
               end_line: usize,
               end_column: usize)
               -> Location {
        Location {
            begin: Position::new(begin_line, begin_column),
            end: Position::new(end_line, end_column),
        }
    }

    pub fn range(left_location: Location, right_location: Location) -> Location {
        Location {
            begin: left_location.begin,
            end: right_location.end,
        }
    }
}

impl Default for Location {
    fn default() -> Location {
        Location::new(1, 1, 1, 1)
    }
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position {
            line: line,
            column: column,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ~ {}", self.begin, self.end)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

