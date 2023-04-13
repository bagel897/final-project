use crate::{ant_grid::AntGrid, coord::Coord};

mod ant_grid;
mod coord;
mod grid_elements;

fn main() {
    let mut grid = AntGrid::new(10, 10);
    println!("{}", grid);
    grid.put_ant(Coord { x: 0, y: 0 });
    grid.put_ant(Coord { x: 0, y: 5 });
    grid.put_ant(Coord { x: 4, y: 0 });
    grid.put_ant(Coord { x: 8, y: 7 });
    grid.put_food(Coord { x: 9, y: 9 });
    println!("{}", grid);
    for i in 0..20 {
        grid.run_round();
        if i % 5 == 0 {
            println!("{}", grid);
        }
    }
}
