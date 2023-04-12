use std::fmt::Debug;

use crate::{ant_grid::AntGrid, coord::Coord};

pub(crate) trait GridElement: Debug {
    fn exists(&self) -> bool;
    fn new(pos: &Coord) -> Self
    where
        Self: Sized;
    fn decide(&mut self, grid: &AntGrid) -> Coord;
}
