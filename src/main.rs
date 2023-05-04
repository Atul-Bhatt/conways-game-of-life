use ggez::{graphics, GameResult, event, Context};

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

#[derive(Copy, Clone)]
struct Grid {
    pub x: i32,
    pub y: i32,
}

impl Grid {
    fn update(&mut self) {
        loop { }
    }

    fn draw(self, canvas: &mut graphics::Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new(1., 1., 150., 150.))
                .color([0., 200., 0., 1.])
        );
    }
}

struct MainState {
    grid: Grid
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ctx.time.check_update_time(FPS) {
            self.grid.update();
            self.draw(ctx)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Create a canvas that renders to the frame
        let mut canvas = graphics::Canvas::from_frame(
            ctx, graphics::Color::from([0.0, 1.0, 0.0, 1.0])
        );

        self.grid.draw(&mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }
}

impl MainState {
    fn new() -> MainState {
        MainState { grid: Grid { x:0, y:0 }}
    }
}

fn main() -> GameResult {

    let (ctx, events_loop) = ggez::ContextBuilder::new("Game of Life", "Atul Bhatt")
        .window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    event::run(ctx, events_loop, MainState::new())
}
