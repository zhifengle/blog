#[derive(Clone, PartialEq)]
pub struct Position {
    // Column
    pub x: u64,
    // Line
    pub y: u64,
}

impl Position {
    pub fn new(x: u64, y: u64) -> Position {
        Position { x, y }
    }
}
