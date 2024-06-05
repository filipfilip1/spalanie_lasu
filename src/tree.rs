#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Tree(TreeState),
}

#[derive(Clone, Copy, PartialEq)]
pub struct TreeState {
    pub burning: bool,
    pub burnt: bool,
    pub health: u8,
}

impl TreeState {
    pub fn new() -> Self {
        TreeState {
            burning: false,
            burnt: false,
            health: 100,
        }
    }
}
