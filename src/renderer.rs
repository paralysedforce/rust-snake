use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use crate::game::{GameState, RunningState};
use crate::geometry::Point;

const DOT_SIZE_IN_PXS: i32 = 20;
pub struct Renderer {
    pub canvas: WindowCanvas,
}

impl Renderer {
    pub fn draw(&mut self, game_state: &GameState) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::GREEN);
        for point in game_state.player.body.iter() {
            self.draw_point(&point)?;
        }

        self.canvas.set_draw_color(Color::RED);
        self.draw_point(&game_state.food.location)?;

        if game_state.running_state == RunningState::Paused {
            self.canvas.set_draw_color(Color::BLUE);
            let logo = paused_logo(&Point(6, 10));
            for point in logo {
                self.draw_point(&point)?;
            }
        }

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

const PAUSE: &str = "\
#### #### #  #  ### #####
#  # #  # #  # #    #     
#### #### #  #  ##  ###   
#    #  # #  #    # #     
#    #  # #### ###  #####";


fn paused_logo(start: &Point) -> Vec<Point> {
    let mut vecs: Vec<Point> = Vec::new();
    for (row, line) in PAUSE.split('\n').enumerate() {
        for (col, val) in line.chars().enumerate() {
            if val == '#' {
                let x = start.0 + (col as i32);
                let y = start.1 + (row as i32);
                vecs.push(Point(x, y));
            }
        }
    }
    vecs
}
