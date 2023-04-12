use crate::{ant_grid::AntGrid, coord::Coord};

mod ant_grid;
mod coord;
mod grid_elements;

fn main() {
    let mut grid = AntGrid::new(10, 10);
    println!("{}", grid);
    grid.put_ant(Coord { x: 0, y: 0 });
    println!("{}", grid);
    grid.run_round();
    println!("{}", grid);
    println!("Hello, world!");
}
