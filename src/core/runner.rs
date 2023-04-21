use std::time::Instant;

use image::Rgb;
use rand::{thread_rng, Rng};

use crate::core::{AntGrid, Coord, Team};

use super::{
    ant_grid::Options,
    grid_elements::{grid_element::GridElement, hive::Hive},
};
pub(crate) trait Runner {
    fn new(rows: usize, cols: usize) -> Self
    where
        Self: Sized;

    fn put(&mut self, elem: dyn GridElement);
    fn set_opts(&mut self, options: Options);
    fn print(&self);
    fn run_dynamic(&mut self) -> usize;
    fn reset(&self);
}
pub(crate) struct BaseRunner {
    pub grid: AntGrid,
    rng: rand::rngs::ThreadRng,
    pub(crate) teams: Vec<Team>,
}
impl Runner for BaseRunner {
    fn new(rows: usize, cols: usize) -> Self {
        let mut res = BaseRunner {
            grid: AntGrid::new(rows, cols),
            rng: thread_rng(),
            teams: Vec::new(),
        };
        res.default_setup();
        return res;
    }
    fn set_opts(&mut self, options: Options) {
        self.grid.options = options;
    }

    fn put(&mut self, elem: dyn GridElement) {
        self.grid.put(elem);
    }
    fn print(&self) {
        println!("{}", self.grid);
    }
    fn run_dynamic(&mut self) -> usize {
        puffin::profile_function!();
        let start = Instant::now();
        let mut n = 0;
        while start.elapsed().as_millis() < (1000.0 / 60.0) as u128 && n < self.grid.options.speed {
            self.grid.run_round();
            n += 1;
        }
        return n;
    }
    fn reset(&self) {
        let (rows, cols) = (self.grid.rows(), self.grid.cols());
        self.grid = AntGrid::new(rows, cols);
        self.teams.clear();
        self.default_setup();
    }
}
impl BaseRunner {
    fn default_setup(&mut self) {
        self.put_team(Rgb([255, 0, 0]), "Red");
        self.put_team(Rgb([255, 0, 255]), "Purple");
        self.put_team(Rgb([255, 255, 0]), "Yellow");
    }
    fn rand_coord(&mut self) -> Coord {
        let x = self.rng.gen_range(0..self.grid.cols());
        let y = self.rng.gen_range(0..self.grid.rows());
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
        self.grid.put(Hive::new(rand, team));
    }
}
