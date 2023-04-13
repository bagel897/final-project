use rand::{distributions::Uniform, Rng};

use crate::{
    coord::Coord,
    grid_elements::{
        ant::{Ant, Team},
        empty::Empty,
        food::Food,
        grid_element::GridElement,
    },
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
    pub rows: usize,
    pub cols: usize,
    smell: f64,
}
impl AntGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        AntGrid {
            grid: HashMap::new(),
            ant_queue: VecDeque::new(),
            food: Vec::new(),
            cols,
            rows,
            smell: 0.5,
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
        let mut other_queue = VecDeque::new();
        while !self.ant_queue.is_empty() {
            let ant = self.ant_queue.pop_front().unwrap();
            let old_pos = ant.borrow().pos().clone();
            let c = ant.borrow_mut().decide(self);
            self.grid.remove(&old_pos);
            if c.is_none() {
                continue;
            }
            self.grid.insert(c.unwrap(), ant.clone());
            other_queue.push_back(ant);
        }
        self.ant_queue = other_queue;
    }
    fn adjust(&self, distance: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let top = 1.0 + self.smell;
        let bot = 1.0 - self.smell;
        let rand = rng.sample(Uniform::new(bot, top));
        return rand * distance;
    }
    pub fn distance_to_food(&self, pt: &Coord) -> Option<f64> {
        if self.is_blocked(pt) {
            return None;
        }
        return self
            .food
            .iter()
            .map(|f| -> f64 {
                return self.adjust(pt.distance(&f.borrow().pos()));
            })
            .min_by(|x, y| x.total_cmp(y));
    }
    pub fn run_round(&mut self) {
        self.run_decide();
    }
    pub fn attack(&self, coord: &Coord, team: &Team) {
        assert!(self.is_enemy(coord, team));
        let ant = self.grid.get(coord).unwrap();
        let mut other_entity = ant.borrow_mut();
        other_entity.attacked(1);
    }
    pub fn is_enemy(&self, coord: &Coord, team: &Team) -> bool {
        if !self.does_exist(coord) {
            return false;
        }
        let ant = self.grid.get(coord);
        if ant.is_none() {
            return false;
        }
        let other_team = ant.unwrap().borrow().team();
        return match other_team {
            None => false,
            Some(t) => &t != team,
        };
    }
    pub fn put_ant(&mut self, pos: Coord, team: &Team) {
        assert!(self.does_exist(&pos));
        if self.is_blocked(&pos) {
            return;
        }
        let ant = Rc::new(RefCell::new(Ant::new(&pos, team)));
        self.grid.insert(pos, ant.clone());
        self.ant_queue.push_back(ant);
    }
    pub fn put_food(&mut self, pos: Coord) {
        assert!(self.does_exist(&pos));
        if self.is_blocked(&pos) {
            return;
        }
        let food = Rc::new(RefCell::new(Food::new(&pos)));
        self.grid.insert(pos, food.clone());
        self.food.push(food.clone());
        self.ant_queue.push_back(food);
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
