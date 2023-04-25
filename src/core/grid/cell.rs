use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use image::Rgb;

use crate::core::grid_elements::{empty::Empty, grid_element::GridElement};

use super::Team;

#[derive(Clone)]
pub(crate) struct Cell {
    pub elem: Option<Rc<RefCell<dyn GridElement>>>,
    pub pheremones: HashMap<(Team, bool), usize>,
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
            pheremones: HashMap::new(),
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
                return if self.pheremones.len() > 0 {
                    Rgb([10, 10, 10])
                } else {
                    Rgb([0, 0, 0])
                }
            }
        }
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
