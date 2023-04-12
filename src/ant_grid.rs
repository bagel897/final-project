use crate::{
    coord::Coord,
    grid_elements::{ant::Ant, grid_element::GridElement},
};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    rc::Rc,
};
pub(crate) struct AntGrid {
    grid: HashMap<Coord, Rc<dyn GridElement>>,
    queue: VecDeque<Box<dyn GridElement>>,
    rows: usize,
    cols: usize,
}
impl AntGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        AntGrid {
            grid: HashMap::new(),
            queue: VecDeque::new(),
            cols,
            rows,
        }
    }
    pub fn is_blocked(&self, coord: &Coord) -> bool {
        let get = self.grid.get(coord);

        return get.map_or(false, |g| return g.exists());
    }
    fn run_decide(&mut self) {
        let mut other_queue = VecDeque::new();
        while !self.queue.is_empty() {
            let mut ant = self.queue.pop_front().unwrap();
            ant.decide(self);
            other_queue.push_back(ant);
        }
        while !other_queue.is_empty() {
            let ant = other_queue.pop_front().unwrap();
            self.queue.push_back(ant);
        }
    }
    pub fn run_round(&mut self) {
        self.run_decide();
    }
    pub fn put_ant(&mut self, pos: Coord) {
        let ant = Box::new(Ant::new(&pos));
        // self.grid.insert(pos, ant.clone());
        self.queue.push_back(ant);
    }
}
impl Display for AntGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.rows {
            write!(f, "|");
            for y in 0..self.cols {
                if self.grid.contains_key(&Coord { x, y }) {
                    write!(f, " {:?} |", self.grid.get(&Coord { x, y }));
                } else {
                    write!(f, "  |");
                }
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
