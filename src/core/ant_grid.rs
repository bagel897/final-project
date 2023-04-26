use std::{cell::RefCell, collections::VecDeque, fmt::Display, rc::Rc};

use multimap::MultiMap;
use rand::distributions::Uniform;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::core::grid::Pheromones;
use crate::core::grid_elements::dirt::DIRT_ELEMENT;
use crate::core::options::Options;
use crate::core::team_element::ElementType::Hive;
use crate::core::{grid::Grid, grid_elements::grid_element::GridElement, Coord, Team};

use super::{
    grid::Export,
    grid_elements::food::FOOD_ELEMENT,
    signals::Signal,
    team_element::{ElementType, TeamElement},
};

pub(crate) struct AntGrid {
    grid: Grid,
    elements: MultiMap<TeamElement, Rc<RefCell<dyn GridElement>>>,
    round_num: usize,
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
    pub(super) fn attack(&mut self, coord: &Coord, team: &Team) {
        assert!(self.is_enemy(coord, team));
        self.damage(coord);
    }

    pub(super) fn pass_food(&mut self, coord: &Coord, pheromones: usize) -> Option<usize> {
        assert!(self.is_blocked(coord));
        return self
            .grid
            .get_mut(coord)
            .get_elem()
            .borrow_mut()
            .pass_food(pheromones);
    }
    fn damage(&mut self, coord: &Coord) {
        let ant = self.grid.get(coord).elem.clone().unwrap();
        let mut other_entity = ant.borrow_mut();
        other_entity.attacked(1);
        if other_entity.is_removed() {
            self.grid.get_mut(other_entity.pos()).elem = None;
            let team_elem = other_entity.team_element();
            drop(other_entity);
            self.elements
                .get_vec_mut(&team_elem)
                .unwrap()
                .drain_filter(|e| e.borrow().is_removed());
        }
    }
    pub(super) fn eat_food(&mut self, coord: &Coord) {
        assert!(self.is_food(coord));
        self.damage(coord);
    }
    pub(super) fn remove_dirt(&mut self, coord: &Coord) {
        assert!(self.is_dirt(coord));
        self.damage(coord);
    }
    pub(super) fn is_dirt(&self, coord: &Coord) -> bool {
        return self.is_same_team_elem(coord, &DIRT_ELEMENT);
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
        return self.is_same_team_elem(coord, &FOOD_ELEMENT);
    }
    pub(super) fn is_hive_same_team(&self, coord: &Coord, team: Team) -> bool {
        return self.is_same_team_elem(
            coord,
            &TeamElement {
                element: ElementType::Hive,
                team: Some(team),
            },
        );
    }
    pub(super) fn is_same_team_elem(&self, coord: &Coord, team_elem: &TeamElement) -> bool {
        if !self.grid.does_exist(coord) {
            return false;
        }
        let ant = self.grid.get(coord).get_elem();
        let elem = ant.borrow().team_element();
        return elem == *team_elem;
    }
    pub(super) fn distance_to_enemy(&self, pt: &Coord, team: &Team) -> Option<f64> {
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
    pub(super) fn hive_exists(&mut self, team: Team) -> bool {
        return self
            .elements
            .get(&TeamElement {
                element: Hive,
                team: Some(team),
            })
            .is_some();
    }
    pub(super) fn get_pheromones(&mut self, pt: &Coord, team: Team, state_bool: bool) -> usize {
        self.grid
            .get_mut(pt)
            .clear_old(self.round_num, self.options.decay);
        let key = (team, state_bool);
        return self
            .grid
            .get(pt)
            .pheromones
            .get(&key)
            .map(|p| p.pheromones)
            .unwrap_or(usize::MAX);
    }
    pub(super) fn put_pheromones(
        &mut self,
        pos: Coord,
        new_val: usize,
        team: &Team,
        state_bool: bool,
    ) {
        self.grid
            .get_mut(&pos)
            .clear_old(self.round_num, self.options.decay);
        let key = (team.clone(), state_bool);
        let old = self.grid.get_mut(&pos).pheromones.get(&key);
        let old_val = old.clone().map(|a| a.pheromones).unwrap_or(usize::MAX);
        if new_val < old_val {
            self.grid.get_mut(&pos).pheromones.insert(
                key,
                Pheromones {
                    pheromones: new_val,
                    age: self.round_num,
                },
            );
        }
    }
}

impl AntGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        AntGrid {
            grid: Grid::new(rows, cols),
            elements: MultiMap::new(),
            options: Options::default(),
            round_num: 0,
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
        self.round_num += 1;
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
