use ggez::{graphics, GameResult, event, Context};
use ggez::nalgebra as na;
use rand::{self, thread_rng, Rng};

const GRID_CELL_SIZE: (f32, f32) = (16., 16.);

fn draw_cell(ctx: &mut Context, cell: na::Point2<f32>) -> GameResult {
    let cell_rect = graphics::Rect::new(
        cell.x,
        cell.y,
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

fn new_random_cells(ctx: &mut Context) -> Vec<Cell> {
    let (screen_w, screen_h) = graphics::drawable_size(&ctx);
    let mut rng = thread_rng();
    let mut random_cells: Vec<Cell> = vec![];

    for _ in 0..50 {
        let mut rand_num_x = (rng.gen::<i32>() as f32 % screen_w/2.).abs();
        let mut rand_num_y = (rng.gen::<i32>() as f32 % screen_h/2.).abs();

        // Push initial points towards center
        if rand_num_x < screen_w/4. {
            rand_num_x += screen_w/4.;
        }

        if rand_num_y < screen_h/4. {
            rand_num_y += screen_h/4.;
        }

        // floor rand_num values to closest 16x16 value
        let x = (rand_num_x - (rand_num_x % 16.)) as f32;
        let y = (rand_num_y - (rand_num_y % 16.)) as f32;

        random_cells.push(Cell::new(x, y));
    }

    random_cells
}

fn check_neighbour(neighbour_coord: Cell, cell_snap: &Vec<Cell>) -> i32 {
    if cell_snap.contains(&neighbour_coord) {
        return 1;
    } else {
        return 0;
    }
}

fn calculate_alive_or_dead(cell: Cell, cell_snap: &Vec<Cell>, new_snap: &mut Vec<Cell>) {
    let mut neighbour_count = 0;

    neighbour_count += check_neighbour(Cell::new(cell.position.x - 16., cell.position.y - 16.), cell_snap);
    neighbour_count += check_neighbour(Cell::new(cell.position.x + 16., cell.position.y - 16.), cell_snap);
    neighbour_count += check_neighbour(Cell::new(cell.position.x - 16., cell.position.y + 16.), cell_snap);
    neighbour_count += check_neighbour(Cell::new(cell.position.x + 16., cell.position.y + 16.), cell_snap);
    neighbour_count += check_neighbour(Cell::new(cell.position.x - 16., cell.position.y), cell_snap);
    neighbour_count += check_neighbour(Cell::new(cell.position.x + 16., cell.position.y), cell_snap);
    neighbour_count += check_neighbour(Cell::new(cell.position.x, cell.position.y + 16.), cell_snap);
    neighbour_count += check_neighbour(Cell::new(cell.position.x, cell.position.y - 16.), cell_snap);

    if cell_snap.contains(&cell) {
        if neighbour_count == 2 || neighbour_count == 3 {
            new_snap.push(cell);
        }
    } else {
        if neighbour_count == 3 {
            new_snap.push(cell);
        }
    }
}

#[derive(Clone)]
struct Cell {
    alive: bool,
    position: na::Point2<f32>,
}

impl Cell {
    fn new(x: f32, y: f32) -> Cell {
        Cell {
            alive: true,
            position: na::Point2::new(x, y),
        }
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.position == other.position;
    }
}


struct MainState {
    cell_vec: Vec<Cell>,
}


impl MainState {
    fn new(ctx: &mut Context) -> Self {
        MainState { 
            cell_vec: new_random_cells(ctx),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let mut new_snap: Vec<Cell> = vec![];

        for x in (0..(screen_h as i32)).step_by(16) {
            for y in (0..(screen_w as i32)).step_by(16) {
                calculate_alive_or_dead(
                Cell::new(x as f32, y as f32),
                &self.cell_vec, &mut new_snap);
            }
        }

        self.cell_vec = new_snap;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for cell in self.cell_vec.clone() {
            draw_cell(ctx, cell.position)?;
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
