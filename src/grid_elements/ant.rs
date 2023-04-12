use crate::{
    ant_grid::AntGrid,
    coord::{Coord, Dir},
};

use super::grid_element::GridElement;
#[derive(Debug)]
pub(crate) struct Ant {
    dir: Dir,
    pos: Coord,
}
impl GridElement for Ant {
    fn new(pos: &Coord) -> Self {
        return Ant {
            dir: Dir::UP,
            pos: pos.clone(),
        };
    }
    fn exists(&self) -> bool {
        return true;
    }
    fn decide(&mut self, grid: &AntGrid) -> Coord {
        if grid.is_blocked(&self.next()) {
            todo!();
        } else {
            return self.next();
        }
    }
}
impl Ant {
    pub(self) fn next(&self) -> Coord {
        return self.pos.next_cell(&self.dir);
    }
}
