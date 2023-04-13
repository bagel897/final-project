use std::fmt::Display;

use crate::coord::Coord;

use super::grid_element::GridElement;

#[derive(Debug)]
pub(crate) struct Food {
    pub pos: Coord,
}
impl GridElement for Food {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn exists(&self) -> bool {
        return true;
    }
    fn new(pos: &Coord) -> Self
    where
        Self: Sized,
    {
        return Food { pos: pos.clone() };
    }
    fn decide(&mut self, grid: &crate::ant_grid::AntGrid) -> Coord {
        return self.pos;
    }
}
impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "f")
    }
}
