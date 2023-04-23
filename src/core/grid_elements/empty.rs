use std::fmt::Display;

use image::Rgb;

use crate::core::{team_element::ElementType, AntGrid, Coord};

use super::grid_element::GridElement;
#[derive(Debug, Clone, Copy)]
pub(crate) struct Empty {}
impl GridElement for Empty {
    fn pos(&self) -> &Coord {
        panic!();
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
    fn type_elem(&self) -> ElementType {
        ElementType::Empty
    }
}
impl Empty {
    pub fn new() -> Self {
        return Empty {};
    }
}
impl Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")
    }
}
