use multimap::MultiMap;
use rand::{distributions::Uniform, thread_rng, Rng};

use crate::core::{grid::Grid, grid_elements::grid_element::GridElement, Coord, Team};
use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};
pub(crate) struct Options {
    pub pheremones_inc: f64,
    pub smell: f64,
    pub starting_food: usize,
    pub signal_radius: f64,
    pub max_pheremones: f64,
    pub speed: usize,
}
impl Default for Options {
    fn default() -> Self {
        return Options {
            pheremones_inc: 1.0,
            smell: 0.5,
            starting_food: 10,
            signal_radius: 10.0,
            max_pheremones: 10.0,
            speed: 20,
        };
    }
}
use super::{
    grid::GridIterator,
    grid_elements::food::FOOD_ELEMENT,
    signals::Signal,
    team_element::{ElementType, TeamElement},
};
pub(crate) struct AntGrid {
    grid: Grid,
    elements: MultiMap<TeamElement, Rc<RefCell<dyn GridElement>>>,
    pub options: Options,
    rng: rand::rngs::ThreadRng,
}
impl AntGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        AntGrid {
            grid: Grid::new(rows, cols),
            elements: MultiMap::new(),
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
        let mut to_iter: VecDeque<Rc<RefCell<dyn GridElement>>> = self
            .elements
            .flat_iter_mut()
            .map(|(_, v)| v.to_owned())
            .collect();
        while !to_iter.is_empty() {
            let ant = to_iter.pop_front().unwrap();
            let old_pos = ant.borrow().pos().clone();
            let c = ant.borrow_mut().decide(self);
            self.grid.get_mut(&old_pos).elem = None;
            if c.is_none() {
                continue;
            }
            self.grid.get_mut(&c.unwrap()).elem = Some(ant.clone());
        }
    }
    fn adjust(&mut self, distance: f64) -> f64 {
        let top = 1.0 + self.options.smell;
        let bot = 1.0 - self.options.smell;
        let rand = self.rng.sample(Uniform::new(bot, top));
        return rand * distance;
    }
    fn distance(&mut self, team_elem: &TeamElement, pt: &Coord) -> Option<f64> {
        if self.is_blocked(pt) {
            return None;
        }
        return Some(
            self.adjust(
                self.elements
                    .get_vec(team_elem)?
                    .iter()
                    .filter_map(|f| f.try_borrow().ok())
                    .map(|f| pt.distance(&f.pos()))
                    .min_by(|x, y| x.total_cmp(y))?,
            ),
        );
    }
    pub fn distance_to_food(&mut self, pt: &Coord) -> Option<f64> {
        return self.distance(&FOOD_ELEMENT, pt);
    }
    pub(super) fn send_signal(&mut self, pt: &Coord, signal: Signal, team_elem: TeamElement) {
        for mut i in self
            .elements
            .get_vec_mut(&team_elem)
            .unwrap()
            .iter()
            .filter_map(|f| f.try_borrow_mut().ok())
            .filter(|f| f.pos().distance(pt) < self.options.signal_radius)
        {
            i.recv_signal(signal);
        }
        match self.elements.get_vec_mut(&TeamElement {
            element: ElementType::Hive,
            team: team_elem.team,
        }) {
            None => (),
            Some(l) => {
                for mut i in l
                    .iter()
                    .filter_map(|f| f.try_borrow_mut().ok())
                    .filter(|f| f.pos().distance(pt) < self.options.signal_radius)
                {
                    i.recv_signal(signal);
                }
            }
        }
    }
    pub fn distance_to_enemy(&mut self, pt: &Coord, team: &Team) -> Option<f64> {
        if self.is_blocked(pt) {
            return None;
        }
        let elems: Vec<TeamElement> = self
            .elements
            .keys()
            .filter(|k| k.team.map_or(false, |t| t != *team))
            .map(|t| t.clone())
            .collect();

        elems
            .iter()
            .map(|elem| self.distance(elem, pt))
            .filter_map(|f| f)
            .min_by(|x, y| x.total_cmp(y))
    }
    pub fn distance_to_hive(&mut self, pt: &Coord, team: &Team) -> Option<f64> {
        if self.is_blocked(pt) {
            return None;
        }
        return self.distance(
            &TeamElement {
                element: ElementType::Hive,
                team: Some(*team),
            },
            pt,
        );
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

        return ant.borrow().type_elem() == ElementType::Food;
    }
    pub fn is_hive_same_team(&self, coord: &Coord, team: Team) -> bool {
        if !self.grid.does_exist(coord) {
            return false;
        }
        let ant = self.grid.get(coord).get_elem(&coord);
        let elem = ant.borrow().team_element();
        return elem
            == TeamElement {
                element: ElementType::Hive,
                team: Some(team),
            };
    }
    pub fn put<T: GridElement>(&mut self, elem: T) {
        let pos = elem.pos().clone();
        let elem_ref = Rc::new(RefCell::new(elem));
        if !self.grid.does_exist(&pos) {
            return;
        }
        if self.is_blocked(&pos) {
            return;
        }
        self.grid.get_mut(&pos).elem = Some(elem_ref.clone());
        self.elements
            .insert(elem_ref.borrow().team_element(), elem_ref.clone());
    }
    pub fn put_pheremones(&mut self, pos: Coord, prev: Coord) {
        self.grid.get_mut(&pos).pheremones = Some(prev);
    }
    pub fn rows(&self) -> usize {
        return self.grid.rows;
    }
    pub fn cols(&self) -> usize {
        return self.grid.cols;
    }

    pub fn iter(&self) -> GridIterator {
        return self.grid.iter();
    }
    pub fn export(&self) -> Grid {
        return self.grid.clone();
    }
}
impl Display for AntGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}
