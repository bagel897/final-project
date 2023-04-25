use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use image::Rgb;

use crate::core::grid_elements::{empty::Empty, grid_element::GridElement};

use super::Team;

#[derive(Clone)]
pub(crate) struct Pheromones {
    pub(crate) pheromones: usize,
    pub(crate) age: usize,
}

#[derive(Clone)]
pub(crate) struct Cell {
    pub elem: Option<Rc<RefCell<dyn GridElement>>>,
    pub pheromones: HashMap<(Team, bool), Pheromones>,
}

impl Default for Cell {
    fn default() -> Self {
        return Cell::new();
    }
}

impl Cell {
    fn new() -> Self {
        return Cell {
            elem: None,
            pheromones: HashMap::new(),
        };
    }
    pub fn get_elem(&self) -> Rc<RefCell<dyn GridElement>> {
        return self
            .elem
            .clone()
            .unwrap_or(Rc::new(RefCell::new(Empty::new())));
    }
    pub fn color(&self) -> Rgb<u8> {
        match &self.elem {
            Some(elem) => elem.borrow().color(),
            None => {
                return if self.pheromones.len() > 0 {
                    Rgb([10, 10, 10])
                } else {
                    Rgb([0, 0, 0])
                };
            }
        }
    }
    pub fn clear_old(&mut self, round_num: usize, decay: usize) {
        self.pheromones
            .drain_filter(|key, val| round_num - val.age > decay);
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.elem.clone() {
            None => Empty::new().fmt(f),
            Some(i) => i.borrow().fmt(f),
        }
    }
}
