extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

use crate::geometry::*;
use crate::renderer::Renderer;
use crate::entities::player::Player;
use crate::entities::food::Food;


const FRAME_RATE: Duration = Duration::new(0, 1_000_000_000u32 / 30);
const REFRESH_RATE: Duration = Duration::new(0, 1_000_000_000u32 / 10);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RunningState { Paused, Running, Exit }

pub struct GameState {
    pub player: Player,
    pub food: Food,
    pub running_state: RunningState,
    input_direction: Direction,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: Player::new(),
            food: Food::new(),
            running_state: RunningState::Paused,
            input_direction: Direction::Right,
        }
    }

    pub fn update(&mut self)
    {
        self.player.update(&self.food, self.input_direction);
        self.food.update(&self.player);

        if self.player.is_dead() {
            self.running_state = RunningState::Exit;
        }
    }
}

pub struct Game {
    sdl_context: Sdl,
    renderer: Renderer,
    game_state: GameState
}


// Game main body
impl Game {
   pub fn new() -> Result<Game, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Rust Snake", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas()
            .build()
            .map_err(|e| e.to_string())?;


        Ok(Game {
            game_state: GameState::new(),
            sdl_context,
            renderer: Renderer{canvas},
        })
   }

   pub fn run(&mut self) -> Result<(), String> {
       let sdl_context = &self.sdl_context;
       let mut event_pump = sdl_context.event_pump()?;
       let mut last_refresh = Instant::now();

       // Main loop
       'main: loop {
           for event in event_pump.poll_iter() {
               self.process_events(event);
           }

           match self.game_state.running_state {
               RunningState::Exit => {
                   println!("Final score: {}", self.game_state.player.score);
                   break 'main;
               },
               RunningState::Running => {
                   let now = Instant::now();
                   let should_update = now.duration_since(last_refresh) > REFRESH_RATE;
                   if should_update {
                       self.game_state.update();
                       last_refresh = now;
                   }
               }
               _ => {}
           }

           self.renderer.draw(&self.game_state)?;
            ::std::thread::sleep(FRAME_RATE);
           }

     
     Ok(())
   }

   fn process_events(&mut self, event: Event) {
       match event {
           Event::Quit { .. }
           | Event::KeyDown {
               keycode: Some(Keycode::Escape),
               ..
           } => {
               self.game_state.running_state = RunningState::Exit;
           },
           Event::KeyDown {
               keycode: Some(Keycode::Down) | Some(Keycode::S),
               ..
           } => {
               if self.game_state.running_state == RunningState::Running {
                   self.game_state.input_direction = Direction::Down;
               }
           },
           Event::KeyDown {
               keycode: Some(Keycode::Right) | Some(Keycode::D),
               ..
           } => {
               if self.game_state.running_state == RunningState::Running {
                   self.game_state.input_direction = Direction::Right;
               }
           },
           Event::KeyDown {
               keycode: Some(Keycode::Left) | Some(Keycode::A),
               ..
           } => {
               if self.game_state.running_state == RunningState::Running {
                   self.game_state.input_direction = Direction::Left;
               }
           },
           Event::KeyDown {
               keycode: Some(Keycode::Up) | Some(Keycode::W),
               ..
           } => {
               if self.game_state.running_state == RunningState::Running {
                   self.game_state.input_direction = Direction::Up;
               }
           },

           
           // Pause
           Event::KeyDown {
               keycode: Some(Keycode::P) | Some(Keycode::Return),
               ..
           } => {
               self.game_state.running_state = match self.game_state.running_state {
                   RunningState::Running => RunningState::Paused,
                   RunningState::Paused => RunningState::Running,
                   RunningState::Exit => RunningState::Exit
               }
           },
           _ => {}
       }
   }

}
