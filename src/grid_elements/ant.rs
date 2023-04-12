use crate::{
    ant_grid::AntGrid,
    coord::{Coord, Dir},
};

use super::grid_element::GridElement;

pub(crate) struct Ant {
    dir: Dir,
}
impl GridElement for Ant {
    fn new() -> Self {
        return Ant { dir: Dir::UP };
    }
    fn exists(&self) -> bool {
        return true;
    }
    fn decide(&mut self, grid: &AntGrid, coord: &Coord) -> Coord {
        if grid.is_blocked(&self.next(coord)) {
            todo!();
        } else {
            return self.next(coord);
        }
    }
}
impl Ant {
    pub(self) fn next(&self, pos: &Coord) -> Coord {
        return pos.next_cell(&self.dir);
    }
}
