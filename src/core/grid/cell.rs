use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::core::grid_elements::{empty::Empty, grid_element::GridElement};

use super::Coord;

#[derive(Clone)]
pub(crate) struct Cell {
    pub elem: Option<Rc<RefCell<dyn GridElement>>>,
    pub pheremones: Option<Coord>,
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
            pheremones: None,
        };
    }
    pub fn get_elem(&self) -> Rc<RefCell<dyn GridElement>> {
        return self
            .elem
            .clone()
            .unwrap_or(Rc::new(RefCell::new(Empty::new())));
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
