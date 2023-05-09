# Conway's Game of Life in Rust

Game of Life was invented by John Conway in 1970. As a zero player game, it requires only initial state to be set and the state changes according to the defined rules.

The grid consists of square cells. Every cell could either be alive or dead. Alive being filled and dead being empty. After every "tick" the grid's state changes according to the following rules --

- If a cell is alive and 2 or 3 of it's 8 neighbors (horizontal, vertical and diagonal) are also alive, the cell remains alive.
- If a cell is alive and it has more than 3 alive neighbours, it dies of overpopulation.
- If a cell is alive and it has fewer than 2 alive neighbours, it dies of underpopultion.
- If a cell is dead and it has exactly 3 neighbours it becomes alive again.

### Drawing Grid and Cells
For implementing the game of life, we need a grid and cells within the grid. Let's import the ggez library and include it into our module.

```shell
cargo add ggez
```

You can also directly mention the crate in ```cargo.toml``` with the required version and features.

```toml
[dependencies]
ggez: "0.5.1"
```

Import the modules we need from ggez.
```rust
use ggez::{graphics, GameResult, event, Context};
use ggez::nalgebra as na;
```

```nalgebra``` is a linear algebra crate which we'll use to define a cartesian point in the grid.

Let's create a context so we can draw on top of it. The ```ggez::ContextBuilder``` will give us a context and event_loop. 

```rust
let cb = ggez::ContextBuilder::new("Game_of_Life", "Atul");
let (mut ctx, mut event_loop) = cb.build()?;
graphics::set_window_title(&ctx, "Conway's Game of Life.");
let mut state = MainState::new(&mut ctx);
event::run(&mut ctx, &mut event_loop, &mut state)
```

In order to constantly draw over the context, we need an initial state that implements ```event::EventHandler```. EventHandler trait has update and draw methods which we'll use to update the state and draw the new set of cells.

```rust
impl event::EventHandler for MainState {
    fn update() -> GameResult<()> {
        todo!()
        Ok(())
    }

    fn draw(ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let cell_rect = graphics::Rect::new(0., 0., 16., 16.);
        let cell_mesh = graphcis::Mesh::new_rectangle(
            ctx,
            graphcis::DrawMode::fill(),
            cell_rect,
            graphics::Color::from_rgb(0, 255, 0), // Green
        ).unwrap();

        let draw_param = graphics::DrawParam::default();
        graphics::draw(ctx, &cell_mesh, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}
```

### Setting Initial State
As we mentioned before, Conway's game requires and initial set of cells to be set alive and then it will propagete on every step. We will create a new struct MainState and which will contain a vector of Cells.

```rust
struct MainSate {
    cell_vec: Vec<Cell>,
}

struct Cell {
    position: na::Point2::new(x, y),
}

```
