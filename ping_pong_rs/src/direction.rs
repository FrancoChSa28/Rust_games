#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Direction {
    pub x: i16,
    pub y: i16
}

impl Direction {
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}