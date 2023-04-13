use std::fmt::Display;

use crate::coord::Coord;

use super::grid_element::GridElement;

use ::colored::Colorize;
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
    fn decide(&mut self, _grid: &mut crate::ant_grid::AntGrid) -> Option<Coord> {
        return Some(self.pos);
    }
    fn is_food(&self) -> bool {
        return true;
    }
}
impl Food {
    pub fn new(pos: &Coord) -> Self {
        return Food { pos: pos.clone() };
    }
}
impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "f".green())
    }
}
