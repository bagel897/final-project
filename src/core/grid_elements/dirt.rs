use std::fmt::Display;

use image::Rgb;

use crate::core::{
    team_element::{ElementType, TeamElement},
    AntGrid, Coord,
};

use super::grid_element::GridElement;
#[derive(Debug)]
pub(crate) struct Dirt {
    pos: Coord,
}
impl GridElement for Dirt {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn exists(&self) -> bool {
        return true;
    }
    fn decide(&mut self, _grid: &mut AntGrid) -> Option<Coord> {
        return Some(self.pos);
    }
    fn color(&self) -> Rgb<u8> {
        return Rgb::from([255, 255, 255]);
    }
    fn type_elem(&self) -> ElementType {
        ElementType::Dirt
    }
}
impl Dirt {
    pub fn new(pos: &Coord) -> Self {
        return Dirt { pos: pos.clone() };
    }
}
impl Display for Dirt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x")
    }
}
pub(super) const DIRT_ELEMENT: TeamElement = TeamElement {
    element: ElementType::Dirt,
    team: None,
};
