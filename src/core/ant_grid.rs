use rand::{distributions::Uniform, thread_rng, Rng};

use crate::core::{
    grid::Grid,
    grid_elements::{ant::Ant, dirt::Dirt, food::Food, grid_element::GridElement, hive::Hive},
    Coord, Team,
};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Display,
    rc::Rc,
};
pub(crate) struct Options {
    pub pheremones_inc: f64,
    pub smell: f64,
    pub starting_food: usize,
    pub signal_radius: f64,
    pub max_pheremones: f64,
}
impl Default for Options {
    fn default() -> Self {
        return Options {
            pheremones_inc: 1.0,
            smell: 0.5,
            starting_food: 10,
            signal_radius: 10.0,
            max_pheremones: 10.0,
        };
    }
}
use super::{grid::GridIterator, signals::Signal};
pub(crate) struct AntGrid {
    grid: Grid,
    ant_queue: VecDeque<Rc<RefCell<dyn GridElement>>>,
    food: Vec<Rc<RefCell<Food>>>,
    hives: HashMap<usize, Vec<Rc<RefCell<Hive>>>>,
    pub options: Options,
    rng: rand::rngs::ThreadRng,
}
impl AntGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        AntGrid {
            grid: Grid::new(rows, cols),
            ant_queue: VecDeque::new(),
            food: Vec::new(),
            hives: HashMap::new(),
            rng: thread_rng(),
            options: Options::default(),
        }
    }

    pub fn is_blocked(&self, coord: &Coord) -> bool {
        if !self.grid.does_exist(coord) {
            return true;
        }
        let get = self.grid.get(coord);
        return get.elem.clone().map_or(false, |g| {
            return g.borrow().exists();
        });
    }
    fn run_decide(&mut self) {
        let mut other_queue = VecDeque::new();
        while !self.ant_queue.is_empty() {
            let ant = self.ant_queue.pop_front().unwrap();
            let old_pos = ant.borrow().pos().clone();
            let c = ant.borrow_mut().decide(self);
            self.grid.get_mut(&old_pos).elem = None;
            if c.is_none() {
                continue;
            }
            self.grid.get_mut(&c.unwrap()).elem = Some(ant.clone());
            other_queue.push_back(ant);
        }
        self.ant_queue = other_queue;
    }
    fn adjust(&mut self, distance: f64) -> f64 {
        let top = 1.0 + self.options.smell;
        let bot = 1.0 - self.options.smell;
        let rand = self.rng.sample(Uniform::new(bot, top));
        return rand * distance;
    }
    pub fn distance_to_food(&mut self, pt: &Coord) -> Option<f64> {
        if self.is_blocked(pt) {
            return None;
        }
        let p = self.adjust(self.grid.get(pt).pheremones);
        let val = self.adjust(
            self.food
                .iter()
                .map(|f| -> f64 {
                    return pt.distance(&f.borrow().pos());
                })
                .min_by(|x, y| x.total_cmp(y))?,
        );
        return Some(val - f64::min(p, self.options.max_pheremones));
    }
    pub(super) fn send_signal(&mut self, pt: &Coord, signal: Signal, team: Team) {
        for mut i in self
            .ant_queue
            .iter()
            .filter_map(|f| f.try_borrow_mut().ok())
            .filter(|f| f.pos().distance(pt) < self.options.signal_radius)
            .filter(|f| f.team().map_or(false, |t| t == team))
        {
            i.recv_signal(signal);
        }
    }
    pub fn distance_to_enemy(&mut self, pt: &Coord, team: &Team) -> Option<f64> {
        if self.is_blocked(pt) {
            return None;
        }
        Some(
            self.adjust(
                self.ant_queue
                    .iter()
                    .filter(|f| {
                        f.try_borrow()
                            .map_or(false, |f| f.team().map_or(false, |g| g != *team))
                    })
                    .map(|f| -> f64 {
                        return pt.distance(&f.borrow().pos());
                    })
                    .min_by(|x, y| x.total_cmp(y))?,
            ),
        )
    }
    pub fn distance_to_hive(&mut self, pt: &Coord, team: &Team) -> Option<f64> {
        if self.is_blocked(pt) {
            return None;
        }
        match self.hives.get(&team.id) {
            None => None,

            Some(i) => Some(
                self.adjust(
                    i.iter()
                        .map(|f| -> f64 {
                            return pt.distance(&f.borrow().pos());
                        })
                        .min_by(|x, y| x.total_cmp(y))?,
                ),
            ),
        }
    }
    pub fn run_round(&mut self) {
        self.run_decide();
    }

    pub fn attack(&self, coord: &Coord, team: &Team) {
        assert!(self.is_enemy(coord, team));
        let ant = self.grid.get(coord).elem.clone().unwrap();
        let mut other_entity = ant.borrow_mut();
        other_entity.attacked(1);
    }
    pub fn is_enemy(&self, coord: &Coord, team: &Team) -> bool {
        if !self.grid.does_exist(coord) {
            return false;
        }
        let ant = &self.grid.get(coord).elem;

        let other_team = match ant {
            None => None,
            Some(a) => a.borrow().team(),
        };
        return match other_team {
            None => false,
            Some(t) => &t != team,
        };
    }
    pub fn is_food(&self, coord: &Coord) -> bool {
        if !self.grid.does_exist(coord) {
            return false;
        }
        let ant = self.grid.get(coord).get_elem(&coord);

        return ant.borrow().is_food();
    }
    pub fn is_hive_same_team(&self, coord: &Coord, team: Team) -> bool {
        if !self.grid.does_exist(coord) {
            return false;
        }
        let ant = self.grid.get(coord).get_elem(&coord);

        return ant.borrow().is_hive() && ant.borrow().team().map_or(false, |t| t == team);
    }
    pub fn put_ant(&mut self, pos: Coord, team: &Team) {
        self.put(pos, Rc::new(RefCell::new(Ant::new(&pos, team))));
    }
    pub fn put_hive(&mut self, pos: Coord, team: Team) {
        let hive = Rc::new(RefCell::new(Hive::new(
            pos,
            team,
            self.options.starting_food,
        )));
        if self.put(pos, hive.clone()) {
            let id = hive.borrow().team().unwrap().id;
            self.hives
                .entry(id)
                .or_insert_with(|| Vec::new())
                .push(hive);
        }
    }
    fn put(&mut self, pos: Coord, elem: Rc<RefCell<dyn GridElement>>) -> bool {
        if !self.grid.does_exist(&pos) {
            return false;
        }
        if self.is_blocked(&pos) {
            return false;
        }
        self.grid.get_mut(&pos).elem = Some(elem.clone());
        self.ant_queue.push_back(elem);
        return true;
    }
    pub fn put_food(&mut self, pos: Coord) {
        let food = Rc::new(RefCell::new(Food::new(&pos)));
        if self.put(pos, food.clone()) {
            self.food.push(food);
        }
    }
    pub fn put_pheremones(&mut self, pos: Coord) {
        self.grid.get_mut(&pos).pheremones += self.options.pheremones_inc
    }
    pub fn rows(&self) -> usize {
        return self.grid.rows;
    }
    pub fn cols(&self) -> usize {
        return self.grid.cols;
    }

    pub fn put_dirt(&mut self, pos: Coord) {
        let dirt = Rc::new(RefCell::new(Dirt::new(&pos)));
        self.put(pos, dirt);
    }
    pub fn iter(&self) -> GridIterator {
        return self.grid.iter();
    }
}
impl Display for AntGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}
