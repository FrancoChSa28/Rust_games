#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}