#[derive(Debug)]
pub struct Cell {
    pub visited: bool,
    pub top_wall : bool,
    pub bottom_wall : bool,
    pub left_wall : bool,
    pub right_wall : bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            visited: false,
            top_wall: true,
            bottom_wall: true,
            left_wall: true,
            right_wall: true,
        }
    }
}
