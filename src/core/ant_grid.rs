use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};

use multimap::MultiMap;
use rand::distributions::Uniform;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::core::{grid::Grid, grid_elements::grid_element::GridElement, Coord, Team};

use super::{
    grid::Export,
    grid_elements::food::FOOD_ELEMENT,
    signals::Signal,
    team_element::{ElementType, TeamElement},
};

#[derive(Clone, Copy)]
pub(crate) struct Options {
    pub pheremones_inc: f64,
    pub smell: f64,
    pub starting_food: usize,
    pub signal_radius: f64,
    pub max_pheremones: f64,
    pub dirt_penalty: f64,
    pub speed: usize,
    pub propogation: usize,
}

impl Default for Options {
    fn default() -> Self {
        return Options {
            pheremones_inc: 1.0,
            smell: 0.5,
            starting_food: 10,
            signal_radius: 2.0,
            max_pheremones: 10.0,
            speed: 20,
            propogation: 3,
            dirt_penalty: 1.2,
        };
    }
}

pub(crate) struct AntGrid {
    grid: Grid,
    elements: MultiMap<TeamElement, Rc<RefCell<dyn GridElement>>>,
    pub options: Options,
}

impl AntGrid {
    pub(super) fn is_blocked(&self, coord: &Coord) -> bool {
        if !self.grid.does_exist(coord) {
            return true;
        }
        let get = self.grid.get(coord);
        return get.elem.clone().map_or(false, |g| {
            return g.borrow().exists();
        });
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
    pub(super) fn attack(&self, coord: &Coord, team: &Team) {
        assert!(self.is_enemy(coord, team));
        let ant = self.grid.get(coord).elem.clone().unwrap();
        let mut other_entity = ant.borrow_mut();
        other_entity.attacked(1);
    }
    pub(super) fn remove_dirt(&self, coord: &Coord) {
        assert!(self.is_dirt(coord));
        let ant = self.grid.get(coord).elem.clone().unwrap();
        let mut other_entity = ant.borrow_mut();
        other_entity.attacked(1);
    }
    pub(super) fn is_dirt(&self, coord: &Coord) -> bool {
        if !self.grid.does_exist(coord) {
            return false;
        }
        return self.grid.get(coord).elem.clone().map_or(false, |f| {
            f.try_borrow()
                .map_or(false, |g| g.type_elem() == ElementType::Dirt)
        });
    }
    pub(super) fn is_enemy(&self, coord: &Coord, team: &Team) -> bool {
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
    pub(super) fn is_food(&self, coord: &Coord) -> bool {
        if !self.grid.does_exist(coord) {
            return false;
        }
        let ant = self.grid.get(coord).get_elem();

        return ant.borrow().type_elem() == ElementType::Food;
    }
    pub(super) fn is_hive_same_team(&self, coord: &Coord, team: Team) -> bool {
        if !self.grid.does_exist(coord) {
            return false;
        }
        let ant = self.grid.get(coord).get_elem();
        let elem = ant.borrow().team_element();
        return elem
            == TeamElement {
                element: ElementType::Hive,
                team: Some(team),
            };
    }
    pub(super) fn distance_to_food(&mut self, pt: &Coord) -> Option<f64> {
        return self.distance(&FOOD_ELEMENT, pt);
    }
    pub(super) fn distance_to_enemy(&mut self, pt: &Coord, team: &Team) -> Option<f64> {
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
    pub(super) fn distance_to_hive(&mut self, pt: &Coord, team: &Team) -> Option<f64> {
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
    pub(super) fn get_pheremones(&self, pt: &Coord, team: &Team) -> f64 {
        return *self.grid.get(pt).pheremones.get(team).unwrap_or(&0.0);
    }
}

impl AntGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        AntGrid {
            grid: Grid::new(rows, cols),
            elements: MultiMap::new(),
            options: Options::default(),
        }
    }

    pub fn run_round(&mut self) {
        let mut to_iter: VecDeque<(usize, Rc<RefCell<dyn GridElement>>)> = self
            .elements
            .keys()
            .filter(|e| {
                e.element != ElementType::Dirt
                    && e.element != ElementType::Empty
                    && e.element != ElementType::Food
            })
            .map(|t| t.clone())
            .map(|k| {
                self.elements
                    .get_vec(&k)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(k, v)| (k, v.to_owned()))
                    .collect::<VecDeque<(usize, Rc<RefCell<dyn GridElement>>)>>()
            })
            .flatten()
            .collect();
        while !to_iter.is_empty() {
            let (_idx, ant) = to_iter.pop_front().unwrap();
            let old_pos = ant.borrow().pos().clone();
            self.grid.get_mut(&old_pos).elem = None;
            if !ant.borrow().is_removed() {
                let c = ant.borrow_mut().decide(self);
                if !ant.borrow().is_removed() {
                    self.grid.get_mut(&c).elem = Some(ant);
                }
            }
        }
        let keys: Vec<TeamElement> = self.elements.keys().map(|f| f.to_owned()).collect();
        for key in keys {
            self.elements
                .get_vec_mut(&key)
                .unwrap()
                .drain_filter(|f| f.borrow().is_removed());
        }
    }
    pub fn put<T: GridElement + 'static>(&mut self, elem: T) {
        self.put_raw(Rc::new(RefCell::new(elem)));
    }
    pub fn put_raw(&mut self, elem_ref: Rc<RefCell<dyn GridElement>>) {
        let pos = elem_ref.borrow().pos().clone();
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
    pub fn put_pheremones(&mut self, pos: Coord, val: f64, team: Team) {
        let old_val = self
            .grid
            .get_mut(&pos)
            .pheremones
            .get(&team)
            .unwrap_or(&0.0);
        let new_val = f64::max(*old_val, val);
        self.grid.get_mut(&pos).pheremones.insert(team, new_val);
    }
    pub fn rows(&self) -> usize {
        return self.grid.rows;
    }
    pub fn cols(&self) -> usize {
        return self.grid.cols;
    }
    pub fn export(&self, frames: usize, teams: Vec<Team>) -> Export {
        return self.grid.export(frames, teams);
    }
}

impl AntGrid {
    fn adjust(&self, distance: f64) -> f64 {
        let mut rng = SmallRng::from_entropy();
        let top = 1.0 + self.options.smell;
        let bot = 1.0 - self.options.smell;
        let rand = rng.sample(Uniform::new(bot, top));
        return rand * distance;
    }
    fn distance(&self, team_elem: &TeamElement, pt: &Coord) -> Option<f64> {
        if self.is_blocked(pt) {
            return None;
        }
        let dirt_factor = match self.is_dirt(pt) {
            true => self.options.dirt_penalty,
            false => 1.0,
        };
        return Some(
            self.adjust(
                self.elements
                    .get_vec(team_elem)?
                    .iter()
                    .filter_map(|f| f.try_borrow().ok())
                    .map(|f| pt.distance(&f.pos()))
                    .min_by(|x, y| x.total_cmp(y))?,
            ) * dirt_factor,
        );
    }
}

impl Display for AntGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}
