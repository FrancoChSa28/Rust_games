use crate::{point::Point, command::Command};
#[derive(Debug)]
pub struct Player {
    position: Point,
    height: u16,
    direction: Command,
    is_moving: bool,
}

impl Player {
    pub fn new(x: u16, y: u16, h: u16) -> Self {
        Self {
            position: Point::new(x, y),
            height: h,
            direction: Command::Down,
            is_moving: false,
        }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn set_direction(&mut self, direction: Command) {
        self.direction = direction;
    }

    pub fn move_player(&mut self, height: u16) {
        match self.direction {
            Command::Up => {
                if self.position.y > 1 {
                    self.position.y -= 1;
                }
            },
            Command::Down => {
                if self.position.y + self.height <= height {
                    self.position.y += 1;
                }
            },
            _ => {}
        }
    }

    pub fn set_is_moving(&mut self, is_moving: bool) {
        self.is_moving = is_moving;
    }

    pub(crate) fn is_moving(&self) -> bool {
        self.is_moving
    }
}