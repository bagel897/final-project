use crate::{
    coord::Coord,
    grid_elements::{ant::Ant, grid_element::GridElement},
};
use std::fmt::Display;
pub(crate) struct AntGrid {
    grid: Vec<Vec<Option<Ant>>>,
}
impl AntGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        AntGrid {
            grid: vec![vec![&Option::<Ant>::None; rows]; cols],
        }
    }
    fn get(&self, coord: &Coord) -> Option<Ant> {
        let row = self.get(coord.x);
        todo!();
    }
    pub fn is_blocked(&self, coord: &Coord) -> bool {
        let get = self.get(coord);

        return get.map_or(false, |g| return g.exists());
    }
}
impl Display for AntGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.fmt(&self, self.grid);
    }
}
