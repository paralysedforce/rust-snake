extern crate sdl2;

use crate::game::Game;

mod game;

pub fn main() -> Result<(), String> {

    let mut game = Game::new()?;
    game.run()
}

