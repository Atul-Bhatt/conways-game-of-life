use ggez::{graphics, GameResult, event, Context};
use nalgebra as na;

const GRID_SIZE: (f32, f32) = (30., 20.);
const GRID_CELL_SIZE: (f32, f32) = (32., 32.);

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
    cell_pos: na::Point2<f32>,
}


impl MainState {
    fn new(ctx: &mut Context) -> Self {
        MainState { 
            cell_pos: na::Point2::new(10., 10.),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let cell_rect = graphics::Rect::new(
            10.,
            10.,
            GRID_CELL_SIZE.0,
            GRID_CELL_SIZE.1,
         );

        let cell_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            cell_rect,
            graphics::WHITE,
        )?;

        let draw_param = graphics::DrawParam::default();
        graphics::draw(ctx, &cell_mesh, draw_param)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Game_of_Life", "Atul");
    let (mut ctx, mut event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "Conway's Game of Life.");
    let mut state = MainState::new(&mut ctx);
    event::run(&mut ctx, &mut event_loop, &mut state)
}
