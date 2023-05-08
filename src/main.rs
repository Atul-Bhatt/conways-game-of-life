use ggez::{graphics, GameResult, event, Context};
use ggez::nalgebra as na;

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (f32, f32) = (16., 16.);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const FPS: f32 = 120.;

fn draw_cell(ctx: &mut Context, cell: Cell) -> GameResult {
    let cell_rect = graphics::Rect::new(
        cell.cell_pos.x,
        cell.cell_pos.y,
        GRID_CELL_SIZE.0,
        GRID_CELL_SIZE.1,
     );

    let cell_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        cell_rect,
        graphics::Color::from_rgb(0, 255, 0),
    ).unwrap();

    let draw_param = graphics::DrawParam::default();
    graphics::draw(ctx, &cell_mesh, draw_param)?;

    Ok(())
}

#[derive(Clone, Debug)]
struct Cell {
    alive: bool,
    cell_pos: na::Point2<f32>
}

impl Cell {
    fn new(pos: na::Point2<f32>) -> Self {
        Cell {
            alive: true,
            cell_pos: pos,
        }
    }
}

struct MainState {
    curr_cell: na::Point2<f32>,
    cell_vec: Vec<Cell>,
}


impl MainState {
    fn new(ctx: &mut Context) -> Self {
        MainState { 
            curr_cell: na::Point2::new(GRID_CELL_SIZE.0, GRID_CELL_SIZE.1),
            cell_vec: vec![],
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.curr_cell.x += GRID_CELL_SIZE.0;
        self.cell_vec.push(Cell::new(self.curr_cell.clone()));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for cell in self.cell_vec.clone() {
            draw_cell(ctx, cell)?;
        }
        
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
