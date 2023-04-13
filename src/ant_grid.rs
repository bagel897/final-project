use crate::{
    coord::Coord,
    grid_elements::{ant::Ant, empty::Empty, food::Food, grid_element::GridElement},
};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Display,
    rc::Rc,
};
pub(crate) struct AntGrid {
    grid: HashMap<Coord, Rc<RefCell<dyn GridElement>>>,
    ant_queue: VecDeque<Rc<RefCell<dyn GridElement>>>,
    food: Vec<Rc<RefCell<Food>>>,
    rows: usize,
    cols: usize,
}
impl AntGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        AntGrid {
            grid: HashMap::new(),
            ant_queue: VecDeque::new(),
            food: Vec::new(),
            cols,
            rows,
        }
    }
    pub fn does_exist(&self, coord: &Coord) -> bool {
        if coord.x >= self.cols || coord.y >= self.rows {
            return false;
        }
        return true;
    }
    pub fn is_blocked(&self, coord: &Coord) -> bool {
        if !self.does_exist(coord) {
            return true;
        }
        let get = self.grid.get(coord);

        return get.map_or(false, |g| {
            return g.borrow().exists();
        });
    }
    fn run_decide(&mut self) {
        let mut new_grid = HashMap::new();
        let mut other_queue = VecDeque::new();
        while !self.ant_queue.is_empty() {
            let ant = self.ant_queue.pop_front().unwrap();
            let c = ant.borrow_mut().decide(self);
            new_grid.insert(c, ant.clone());
            other_queue.push_back(ant);
        }
        while !other_queue.is_empty() {
            let ant = other_queue.pop_front().unwrap();
            self.ant_queue.push_back(ant);
        }
        for f in self.food.iter() {
            let c = f.borrow_mut().decide(self);
            new_grid.insert(c, f.clone());
        }
        self.grid = new_grid;
    }
    pub fn run_round(&mut self) {
        self.run_decide();
    }
    pub fn put_ant(&mut self, pos: Coord) {
        assert!(self.does_exist(&pos));
        let ant = Rc::new(RefCell::new(Ant::new(&pos)));
        self.grid.insert(pos, ant.clone());
        self.ant_queue.push_back(ant);
    }
    pub fn put_food(&mut self, pos: Coord) {
        assert!(self.does_exist(&pos));
        let food = Rc::new(RefCell::new(Food::new(&pos)));
        self.grid.insert(pos, food.clone());
        self.food.push(food);
    }
}
impl Display for AntGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            let y = self.rows - i - 1;
            write!(f, "|")?;
            for x in 0..self.cols {
                write!(f, " ")?;

                match self.grid.get(&Coord { x, y }) {
                    None => Empty::new(&Coord { x, y }).fmt(f),
                    Some(i) => i.borrow().fmt(f),
                }?;
                write!(f, " |")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
