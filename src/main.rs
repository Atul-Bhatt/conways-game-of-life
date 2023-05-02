use ggez::{GameResult, event, Context};

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

struct InitialState {
    pub x: i32,
    pub y: i32,
}

impl event::EventHandler for InitialState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

impl InitialState {
    fn new() -> InitialState {
        InitialState { x: 0, y: 0}
    }
}

fn main() -> GameResult {

    let (ctx, events_loop) = ggez::ContextBuilder::new("Game of Life", "Atul Bhatt")
        .window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    event::run(ctx, events_loop, InitialState::new())
}
