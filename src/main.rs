use crate::game::Game;

mod game;
mod geometry;
mod renderer;
mod entities;

pub fn main() -> Result<(), String> {

    let mut game = Game::new()?;
    game.run()
}

