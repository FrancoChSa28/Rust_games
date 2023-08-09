use std::{io::Stdout, time::{Instant, Duration}};

use crossterm::{terminal::{size, enable_raw_mode, SetSize, ClearType, Clear, disable_raw_mode}, ExecutableCommand, cursor::{Hide, MoveTo, Show}, style::{SetForegroundColor, Print, ResetColor}, event::{poll, read, Event, KeyEvent, KeyCode, KeyModifiers}};

use crate::{command::Command, player::Player, point::Point, ball::Ball};

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

#[derive(Debug)]
pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
    speed: u16,
    player: Player,
    ball: Ball,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size: (u16, u16) = size().unwrap();
        Self {
            stdout,
            original_terminal_size,
            width,
            height,
            speed: 1,
            player: Player::new(2, 1 + (height / 3) as u16, height / 3 as u16),
            ball: Ball::new( (width / 2) as u16, (height / 2) as u16),
        }
    }

    pub fn run(&mut self){
        self.prepare_ui();
        self.render();

        let mut done:bool = false;
        while !done {
            let interval: Duration = self.calculate_interval();
            // let direction = self.snake.get_direction();
            let now = Instant::now();

            while now.elapsed() < interval {
                if let Some(command) = self.get_command(interval - now.elapsed()) {
                    match command {
                        Command::Quit => {
                            done = true;
                            break;
                        },
                        Command::Down | Command::Up => {
                            self.player.set_direction(command);
                            self.player.set_is_moving(true);
                        }
                    }
                }
            }

            if !done {
                if self.player.is_moving() {
                    self.player.move_player(self.height);
                    self.player.set_is_moving(false);
                }
                // Move ball
                self.ball.move_ball(self.width, self.height);
                self.render();
            }
        }

        self.restore_ui();
    }

    fn prepare_ui(&mut self){
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Hide).unwrap();
    }

    fn render(&mut self){
        self.draw_borders();
        self.draw_background();
        self.draw_player();
        self.draw_ball();
    }

    fn draw_borders(&mut self){
        self.stdout.execute(SetForegroundColor(crossterm::style::Color::DarkGrey)).unwrap();

        for y in 0..self.height + 2 {
            self.stdout
                .execute(MoveTo(0, y)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(self.width + 1, y)).unwrap()
                .execute(Print("#")).unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .execute(MoveTo(x, 0)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(x, self.height + 1)).unwrap()
                .execute(Print("#")).unwrap();
        }

        self.stdout
            .execute(MoveTo(0, 0)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(self.width + 1, self.height + 1)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(self.width + 1, 0)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(0, self.height + 1)).unwrap()
            .execute(Print("#")).unwrap();
    }

    fn calculate_interval(&self) -> Duration {
        let speed = MAX_SPEED - self.speed;
        Duration::from_millis(
            (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64
        )
    }

    fn restore_ui(&mut self){
        let (cols, rows) = self.original_terminal_size;
        self.stdout
            .execute(SetSize(cols, rows)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Show).unwrap()
            .execute(ResetColor).unwrap();
        disable_raw_mode().unwrap();
    }

    fn get_command(&self, wait_for: Duration) -> Option<Command> {
        let key_event = self.wait_for_key_event(wait_for)?;

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Command::Quit),
            KeyCode::Char('c') | KeyCode::Char('C') =>
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Some(Command::Quit)
                } else {
                    None
                },
            KeyCode::Up => Some(Command::Up),
            KeyCode::Down => Some(Command::Down),
            _ => None
        }
    }

    fn wait_for_key_event(&self, wait_for: Duration) -> Option<KeyEvent> {
        if poll(wait_for).ok()? {
            let event = read().ok()?;
            if let Event::Key(key_event) = event {
                return Some(key_event);
            }
        }

        None
    }

    fn draw_background(&mut self){
        self.stdout.execute(ResetColor).unwrap();

        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                self.stdout
                    .execute(MoveTo(x, y)).unwrap()
                    .execute(Print(" ")).unwrap();
            }
        }
    }

    fn draw_player(&mut self){
        let position: &Point = self.player.get_position();
        let height: u16 = self.player.get_height();
        for y in position.y..position.y + height {
            self.stdout
                .execute(MoveTo(position.x, y)).unwrap()
                .execute(Print("o")).unwrap();
        }
    }

    fn draw_ball(&mut self) {
        let position: &Point = self.ball.get_position();
        self.stdout
            .execute(MoveTo(position.x, position.y)).unwrap()
            .execute(Print("x")).unwrap();
    }


}