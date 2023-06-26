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
fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Game_of_Life", "Atul");
    let (mut ctx, mut event_loop) = cb.build()?;

    graphics::set_window_title(&ctx, "Conway's Game of Life.");
    
    let mut state = MainState::new(&mut ctx);
    event::run(&mut ctx, &mut event_loop, &mut state)
}
```

In order to constantly draw over the context, we need an initial state that implements ```event::EventHandler```. EventHandler trait has update and draw methods which we'll use to update the state and draw the new set of cells.

```rust
impl event::EventHandler for MainState {
    fn update() -> GameResult<()> {
        todo!()
        Ok(())
    }

    fn draw(ctx: &mut Context) -> GameResult<()> {
        todo!()
        Ok(())
    }
}
```

### Setting Initial State
As we mentioned before, Conway's game requires and initial set of cells to be set alive and then it will propagate on every step. We will create a new struct MainState and which will contain a vector of Cells.

```rust
struct MainSate {
    cell_vec: Vec<Cell>,
}

impl MainState {
    fn new(ctx: &mut)
}

```

For initil setup, we need some random cells to be generated and we'll try to push them to the center so the cells could have neighbours which will allow proper propagation on every step.

```rust
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
```

### Updating Frames
So far we have created an state on which the rules of Game of life will be applied. Now we need to implement our update and draw functions.
The update function will capture the screen size, loop through every cell and calculate the total number of neighbouring cells which are alive. For every alive cell we'll push the respetive coordinate to a vector which we'll use to draw on to the grid.

```rust
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
```

We're almost there, the only thing remaining to do is to implement the utility functions -- ```draw_cell()``` (take a coordinate and context as input and draw it onto the grid), ```check_neighbour()``` (check whether the current snapshot vector contains that coordinate), and ```calculate_alive_or_dead()``` (check all eight coordinates around a cell and call check_neighbours on it).

```rust
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

fn check_neighbour(neighbour_coord: Cell, cell_snap: &Vec<Cell>) -> i32 {
    if cell_snap.contains(&neighbour_coord) {
        return 1;
    } else {
        return 0;
    }
}

```

That's it. Now we finally implemented the conway's implementation. You can do a ```cargo run``` on your root directory where you have the ```cargo.toml``` file and see the magic.

You can find the whole code in this [gist](https://gist.github.com/Atul-Bhatt-RS/2e207992a1928984e07c26f4b937b2b8). 