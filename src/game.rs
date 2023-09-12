extern crate sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use rand::prelude::*;

use crate::geometry::*;
use crate::renderer::Renderer;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RunningState { Paused, Running, Exit }

pub struct GameState {
    pub food: Point,
    pub player: VecDeque<Point>,
    pub running_state: RunningState,
    input_direction: Direction,
    player_direction: Direction,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: VecDeque::from([Point(1,1), Point(2, 1), Point(3, 1)]),
            food: Point(3, 3),
            running_state: RunningState::Paused,
            player_direction: Direction::Right,
            input_direction: Direction::Right,
        }
    }
}

pub struct Game {
    sdl_context: Sdl,
    renderer: Renderer,
    game_state: GameState
}

const FRAME_RATE: Duration = Duration::new(0, 1_000_000_000u32 / 30);
const REFRESH_RATE: Duration = Duration::new(0, 1_000_000_000u32 / 10);

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
                   println!("Final score: {}", self.game_state.player.len());
                   break 'main;
               },
               RunningState::Running => {
                   let now = Instant::now();
                   let should_tick = now.duration_since(last_refresh) > REFRESH_RATE;
                   if should_tick {
                       self.tick();
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

   fn tick(&mut self) -> () {
       match self.game_state.running_state {
           RunningState::Running => {
               if self.game_state.input_direction != opposite(&self.game_state.player_direction){
                   self.game_state.player_direction = self.game_state.input_direction;
               }

               let head: Point = self.game_state.player.back().unwrap().clone();
               let next_move = match self.game_state.player_direction {
                   Direction::Up => Point(0, -1),
                   Direction::Down => Point(0, 1),
                   Direction::Left => Point(-1, 0),
                   Direction::Right => Point(1, 0)
               };
               let next_location = head + next_move;
               let self_intersects = self.game_state.player.contains(&next_location);

               if in_bounds(&next_location) && !self_intersects {
                   let food_eaten = next_location == self.game_state.food;
                   if food_eaten {
                       let mut rng = thread_rng();
                       loop {
                           let food_x = rng.gen_range(1..GRID_X_SIZE);
                           let food_y = rng.gen_range(1..GRID_Y_SIZE);
                           let food = Point(food_x, food_y);

                           if !self.game_state.player.contains(&food){
                               self.game_state.food = food;
                               break;
                           }
                       }
                   }

                   else {
                       self.game_state.player.pop_front();
                   }
                   self.game_state.player.push_back(next_location);
               }
               else {
                   self.game_state.running_state = RunningState::Exit;
               }
           },
           _ => {}
       }
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
               self.game_state.input_direction = Direction::Down;
           },
           Event::KeyDown {
               keycode: Some(Keycode::Right) | Some(Keycode::D),
               ..
           } => {
               self.game_state.input_direction = Direction::Right;
           },
           Event::KeyDown {
               keycode: Some(Keycode::Left) | Some(Keycode::A),
               ..
           } => {
               self.game_state.input_direction = Direction::Left;
           },
           Event::KeyDown {
               keycode: Some(Keycode::Up) | Some(Keycode::W),
               ..
           } => {
               self.game_state.input_direction = Direction::Up;
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
