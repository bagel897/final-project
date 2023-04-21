use std::fmt::Display;

use crate::core::{
    team_element::{ElementType, TeamElement},
    AntGrid, Coord,
};

use super::grid_element::GridElement;

use ::colored::Colorize;
use image::Rgb;
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
    fn decide(&mut self, _grid: &mut AntGrid) -> Option<Coord> {
        return Some(self.pos);
    }
    fn color(&self) -> Rgb<u8> {
        return Rgb::from([0, 255, 0]);
    }
    fn type_elem(&self) -> ElementType {
        ElementType::Food
    }
}
impl Food {
    pub fn new(pos: &Coord) -> Self {
        return Food { pos: pos.clone() };
    }
}
impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "f".green().bold())
    }
}
pub(crate) const FOOD_ELEMENT: TeamElement = TeamElement {
    element: ElementType::Food,
    team: None,
};
