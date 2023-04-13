use crate::{ant_grid::AntGrid, coord::Coord};
use rand::prelude::*;
mod ant_grid;
mod coord;
mod grid_elements;

fn main() {
    let num_ants = 10;
    let num_food = 1;
    let mut rng = rand::thread_rng();
    let cols = 20;
    let rows = cols;
    let num_rounds = 100;
    let interval = 10;
    let mut grid = AntGrid::new(rows, cols);
    println!("{}", grid);
    for _ in 0..num_ants {
        let x = rng.gen_range(0..cols);
        let y = rng.gen_range(0..rows);
        grid.put_ant(Coord { x, y })
    }
    for _ in 0..num_food {
        let x = rng.gen_range(0..cols);
        let y = rng.gen_range(0..rows);
        grid.put_food(Coord { x, y })
    }
    println!("{}", grid);
    for i in 0..num_rounds {
        grid.run_round();
        if i % interval == interval - 1 {
            println!("{}", grid);
        }
    }
}
