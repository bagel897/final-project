use std::{cell::RefCell, rc::Rc, time::Instant};

use image::Rgb;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::core::options::Options;
use crate::core::{AntGrid, Coord, Team};

use super::{
    grid::Export,
    grid_elements::{grid_element::GridElement, hive::Hive},
    Dirt,
};
pub(crate) trait Runner {
    fn put<T: GridElement + 'static>(&mut self, elem: T);
    fn set_opts(&mut self, options: Options);
    fn reset(&mut self);
    fn export(&mut self) -> Export;
    fn run(&mut self) {}
}
pub(crate) struct BaseRunner {
    pub grid: AntGrid,
    pub(crate) teams: Vec<Team>,
    frames: usize,
}
impl Runner for BaseRunner {
    fn put<T: GridElement + 'static>(&mut self, elem: T) {
        self.put_raw(Rc::new(RefCell::new(elem)));
    }

    fn set_opts(&mut self, options: Options) {
        self.grid.options = options;
    }

    fn reset(&mut self) {
        self.frames = 0;
        let (rows, cols) = (self.grid.rows(), self.grid.cols());
        self.grid = AntGrid::new(rows, cols);
        self.teams.clear();
        self.default_setup();
    }
    fn export(&mut self) -> Export {
        return self.grid.export(self.frames, self.teams.clone());
    }
    fn run(&mut self) {
        self.run_dynamic();
    }
}
impl BaseRunner {
    pub fn put_raw(&mut self, elem: Rc<RefCell<dyn GridElement>>) {
        self.grid.put_raw(elem);
    }
    pub fn new(rows: usize, cols: usize, options: Options) -> Self {
        let mut res = BaseRunner {
            grid: AntGrid::new(rows, cols),
            teams: Vec::new(),
            frames: 0,
        };
        res.set_opts(options);
        res.default_setup();
        return res;
    }
    fn default_setup(&mut self) {
        self.put_team(Rgb([255, 0, 0]), "Red");
        self.put_team(Rgb([255, 0, 255]), "Purple");
        self.put_team(Rgb([255, 255, 0]), "Yellow");
        for _ in 0..(self.grid.rows() * self.grid.cols() / 2) {
            let c = self.rand_coord();
            self.grid.put(Dirt::new(&c));
        }
    }
    fn rand_coord(&mut self) -> Coord {
        let mut rng = SmallRng::from_entropy();
        let x = rng.gen_range(0..self.grid.cols());
        let y = rng.gen_range(0..self.grid.rows());
        Coord { x, y }
    }
    fn put_team(&mut self, color: Rgb<u8>, name: &'static str) {
        let team = Team {
            color,
            id: self.teams.len(),
            health: 1,
            name,
        };
        self.teams.push(team);
        let rand = self.rand_coord();
        self.grid
            .put(Hive::new(rand, team, self.grid.options.starting_food));
    }
    pub(crate) fn run_dynamic(&mut self) {
        puffin::profile_function!();
        let start = Instant::now();
        let mut n = 0;
        while start.elapsed().as_millis() < (1000.0 / 60.0) as u128 && n < self.grid.options.speed {
            self.grid.run_round();
            n += 1;
        }
        self.frames += n;
    }
}
