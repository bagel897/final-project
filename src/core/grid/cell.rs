use std::{cell::RefCell, rc::Rc};

use crate::core::grid_elements::grid_element::GridElement;

pub(crate) struct Cell {
    food_dist: Option<f64>,
    elem: Option<Rc<RefCell<dyn GridElement>>>,
    pheremones: f64,
}
impl Default for Cell {
    fn default() -> Self {
        return Cell::new();
    }
}
impl Cell {
    fn new() -> Self {
        return Cell {
            food_dist: None,
            elem: None,
            pheremones: 0.0,
        };
    }
}
