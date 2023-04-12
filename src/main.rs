use crate::ant_grid::AntGrid;

mod ant_grid;
mod coord;
mod grid_elements;

fn main() {
    let grid = AntGrid::new(10, 10);
    println!("{}", grid);
    println!("Hello, world!");
}
