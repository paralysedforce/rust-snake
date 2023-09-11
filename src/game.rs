extern crate sdl2;

use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::ops::Add;
use rand::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(pub i32, pub i32);
impl Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

enum RunningState { Paused, Running, Exit }

#[derive(PartialEq, Copy, Clone)]
enum Direction { Up, Left, Right, Down }

fn opposite(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left
    }
} 

struct GameState {
    food: Point,
    player: VecDeque<Point>,
    running_state: RunningState,
    temp_direction: Direction,
    player_direction: Direction,
    score: i32
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: VecDeque::from([Point(1,1), Point(2, 1), Point(3, 1)]),
            food: Point(3, 3),
            running_state: RunningState::Paused,
            player_direction: Direction::Right,
            temp_direction: Direction::Right,
            score: 3

        }
    }
}


const GRID_X_SIZE: i32 = 40;
const GRID_Y_SIZE: i32 = 30;

fn in_bounds(point: &Point) -> bool {
    let Point(x, y) = point;
    &0 < x && x < &GRID_X_SIZE && &0 < y && y < &GRID_Y_SIZE
}

const DOT_SIZE_IN_PXS: i32 = 20;

struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn draw(&mut self, game_state: &GameState) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        match game_state.running_state {
            RunningState::Running => {
            },
            RunningState::Paused => {
                self.canvas.set_draw_color(Color::BLUE);
                self.draw_point(&Point(GRID_X_SIZE / 2, GRID_Y_SIZE / 2))?;
            },
            _ => {}
        }

        self.canvas.set_draw_color(Color::GREEN);
        for point in game_state.player.iter() {
            self.draw_point(&point)?;
        }

        self.canvas.set_draw_color(Color::RED);
        self.draw_point(&game_state.food)?;

        self.canvas.present();
        Ok(())
    }

    fn draw_point(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
                x * DOT_SIZE_IN_PXS as i32,
                y * DOT_SIZE_IN_PXS as i32,
                DOT_SIZE_IN_PXS as u32,
                DOT_SIZE_IN_PXS as u32
                )
            )?;
        Ok(())
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
                   println!("Final score: {}", self.game_state.score);
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
               if self.game_state.temp_direction != opposite(&self.game_state.player_direction){
                   self.game_state.player_direction = self.game_state.temp_direction;
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
                       self.game_state.score += 1;
                       let mut rng = thread_rng();
                       loop {
                           let food_x = rng.gen_range(1..GRID_X_SIZE);
                           let food_y = rng.gen_range(1..GRID_Y_SIZE);
                           let food = Point(food_x, food_y);

                           if !self.game_state.player.contains(&food){
                               self.game_state.food = food;
                               //println!("{:?}", food);
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
               self.game_state.temp_direction = Direction::Down;
           },
           Event::KeyDown {
               keycode: Some(Keycode::Right) | Some(Keycode::D),
               ..
           } => {
               self.game_state.temp_direction = Direction::Right;
           },
           Event::KeyDown {
               keycode: Some(Keycode::Left) | Some(Keycode::A),
               ..
           } => {
               self.game_state.temp_direction = Direction::Left;
           },
           Event::KeyDown {
               keycode: Some(Keycode::Up) | Some(Keycode::W),
               ..
           } => {
               self.game_state.temp_direction = Direction::Up;
           },

           
           // Pause
           Event::KeyDown {
               keycode: Some(Keycode::P), 
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
