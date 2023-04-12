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
        let next = self.next();
        if next.is_none() || grid.is_blocked(&next.unwrap()) {
            self.dir = self.dir.turn();
            return self.pos;
        } else {
            self.pos = next.unwrap();
            return next.unwrap();
        }
    }
}
impl Ant {
    pub(self) fn next(&self) -> Option<Coord> {
        return self.pos.next_cell(&self.dir);
    }
}
