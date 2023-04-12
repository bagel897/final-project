use crate::coord::Coord;

use super::grid_element::GridElement;

pub(crate) struct Empty {}
impl GridElement for Empty {
    fn exists(&self) -> bool {
        return false;
    }
    fn new() -> Self
    where
        Self: Sized,
    {
        return Empty {};
    }
    fn decide(&mut self, grid: &crate::ant_grid::AntGrid, coord: &Coord) -> Coord {
        return coord.clone();
    }
}
