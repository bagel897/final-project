use std::fmt::Display;

use ::colored::Colorize;
use image::Rgb;

use crate::core::{
    team_element::{ElementType, TeamElement},
    AntGrid, Coord,
};

use super::grid_element::GridElement;

const STARTING_FOOD: usize = 10;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Food {
    pub pos: Coord,
    food: usize,
}

impl GridElement for Food {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn exists(&self) -> bool {
        return true;
    }
    fn decide(&mut self, _grid: &mut AntGrid) -> Coord {
        self.pos
    }
    fn type_elem(&self) -> ElementType {
        ElementType::Food
    }
    fn attacked(&mut self, damage: usize) {
        assert!(!self.is_removed());
        self.food = self.food.saturating_sub(damage);
    }
    fn color(&self) -> Rgb<u8> {
        return Rgb::from([0, 255 * (self.food / STARTING_FOOD) as u8, 0]);
    }
    fn is_removed(&self) -> bool {
        return self.food == 0;
    }
}

impl Food {
    pub fn new(pos: &Coord) -> Self {
        return Food {
            pos: pos.clone(),
            food: STARTING_FOOD,
        };
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
