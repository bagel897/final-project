use crate::coord::Coord;

use super::grid_element::GridElement;
#[derive(Debug)]
pub(crate) struct Empty {
    pos: Coord,
}
impl GridElement for Empty {
    fn exists(&self) -> bool {
        return false;
    }
    fn new(pos: &Coord) -> Self
    where
        Self: Sized,
    {
        return Empty { pos: pos.clone() };
    }
    fn decide(&mut self, grid: &crate::ant_grid::AntGrid) -> Coord {
        return self.pos;
    }
}
