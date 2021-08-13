#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub fn new(col: usize, row: usize) -> Position {
        Position { col, row }
    }
}
