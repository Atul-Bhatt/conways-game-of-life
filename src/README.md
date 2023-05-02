# Conway's Game of Life Written in Rust

Game of Life was invented by John Conway in 1970. As a zero player game, it requires only initial state set and the state changes according to the defined rules.

The grid consists of square cells. Every cell could either be alive or dead. Alive being filled and dead being empty. After every "tick" the grid's state changes according to the following rules --

- If a cell is alive and 2 or 3 of it's 8 neighbors (horizontal, vertical and diagonal) are also alive, the cell remains alive.
- If a cell is alive and it has more than 3 alive neighbours, it dies of overpopulation.
- If a cell is alive and it has fewer than 2 alive neighbours, it dies of underpopultion.
- If a cell is dead and it has exactly 3 neighbours it becomes alive again.