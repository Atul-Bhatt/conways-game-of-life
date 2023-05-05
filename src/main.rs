use ggez::{graphics, GameResult, event, Context};
use nalgebra as na;

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const FPS: u32 = 120;

#[derive(Clone, Debug)]
struct Cell {
    alive: bool,
}

impl Cell {
    fn new(alive: bool) -> Cell {
        Self { alive }
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn set_state(&mut self, state: bool) {
        self.alive = state;
    }
}

struct MainState {
    first_cell: na::Point2<f32>,
}


impl MainState {
    fn new() -> Self {
        MainState { 
            first_cell: na::Point2::new(10., 10.),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

fn main() -> GameResult {
    Ok(())
}
