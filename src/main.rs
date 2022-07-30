mod game;
mod util;

use game::Game;

fn main() {
    let mut game  = Game::new();
    game.start();
}