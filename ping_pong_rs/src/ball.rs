use rand::{Rng, seq::SliceRandom};

use crate::{point::Point, direction::Direction, player::Player};
#[derive(Debug)]
pub struct Ball {
    position: Point,
    direction: Direction,
    character: char,
}

impl Ball {
    pub fn new(x: u16, y: u16) -> Self {
        

        Self {
            position: Point::new(x, y),
            direction: Direction::new(
                vec![-1,1].choose(&mut rand::thread_rng()).unwrap().clone(),
                vec![-1,1].choose(&mut rand::thread_rng()).unwrap().clone()),
            character: 'o',
        }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }

    pub fn move_ball(&mut self, width: u16, height: u16) {
        let new_x: i16 = self.position.x as i16 + self.direction.x;
        let new_y: i16 = self.position.y as i16 + self.direction.y;
        // Verify if the ball is in the left or right border
        if new_x >= 1 && new_x <= (width as i16) {
            self.position.x = new_x as u16;
        } else {
            self.direction.x *= -1;
            return;
        }
        // Verify if the ball is in the top or bottom border
        if new_y >= 1 && new_y <= (height as i16) {
            self.position.y = new_y as u16;
        } else {
            self.direction.y *= -1;
            return;
        }
    }

    pub fn overlaps_with(&mut self, player: &Player){
        let position: &Point = player.get_position();
        let height: u16 = player.get_height();
        if position.y <= self.position.y && self.position.y <= position.y + height {
            // if position.y <= self.position.y && self.position.y <= position.y + height {
            //     self.direction.x *= -1;
            // }
            match rand::thread_rng().gen_range(0, 2){
                0 => self.set_character('o'),
                1 => self.set_character('x'),
                _ => self.set_character('+'),
            }
        }
    }

    pub fn get_character(&self) -> char {
        self.character
    }

    pub fn set_character(&mut self, character: char) {
        self.character = character;
    }
}