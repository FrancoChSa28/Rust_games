use std::{io::stdout, env};

use snake_rs::game::Game;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    Game::new(stdout(), 15, 15).run();
}
