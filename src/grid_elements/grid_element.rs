use crate::{ant_grid::AntGrid, coord::Coord};

pub(crate) trait GridElement {
    fn exists(&self) -> bool;
    fn new() -> Self
    where
        Self: Sized;
    fn decide(&mut self, grid: &AntGrid, coord: &Coord) -> Coord;
}
