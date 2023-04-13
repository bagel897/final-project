use std::fmt::Display;

use crate::coord::Coord;

use super::grid_element::GridElement;
#[derive(Debug)]
pub(crate) struct Empty {
    pos: Coord,
}
impl GridElement for Empty {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn exists(&self) -> bool {
        return false;
    }
    fn decide(&mut self, grid: &crate::ant_grid::AntGrid) -> Coord {
        return self.pos;
    }
}
impl Empty {
    pub fn new(pos: &Coord) -> Self {
        return Empty { pos: pos.clone() };
    }
}
impl Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")
    }
}
