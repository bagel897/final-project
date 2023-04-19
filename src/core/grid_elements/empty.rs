use std::fmt::Display;

use image::Rgb;

use crate::core::{AntGrid, Coord};

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
    fn decide(&mut self, _grid: &mut AntGrid) -> Option<Coord> {
        return None;
    }
    fn color(&self) -> Rgb<u8> {
        return Rgb::from([0, 0, 0]);
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
