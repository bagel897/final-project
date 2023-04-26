use std::fmt::Display;

use image::Rgb;

use crate::core::team_element::TeamElement;
use crate::core::{team_element::ElementType, AntGrid, Coord};

use super::grid_element::GridElement;
#[derive(Debug, Clone, Copy)]
pub(crate) struct Dirt {
    pos: Coord,
    removed: bool,
}
impl GridElement for Dirt {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn exists(&self) -> bool {
        return false;
    }
    fn decide(&mut self, _grid: &mut AntGrid) -> Coord {
        self.pos
    }
    fn type_elem(&self) -> ElementType {
        ElementType::Dirt
    }
    fn attacked(&mut self, _damage: usize) {
        self.removed = true;
    }
    fn color(&self) -> Rgb<u8> {
        return Rgb::from([139, 69, 19]);
    }
    fn is_removed(&self) -> bool {
        return self.removed;
    }
}
impl Dirt {
    pub fn new(pos: &Coord) -> Self {
        return Dirt {
            pos: pos.clone(),
            removed: false,
        };
    }
}
impl Display for Dirt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x")
    }
}
pub(crate) const DIRT_ELEMENT: TeamElement = TeamElement {
    element: ElementType::Dirt,
    team: None,
};
