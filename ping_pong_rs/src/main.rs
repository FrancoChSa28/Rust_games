use std::io::stdout;

use ping_pong_rs::game::Game;

fn main() {
    Game::new(stdout(), 30, 9).run();
}
