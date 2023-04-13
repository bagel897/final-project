use colored::Color;
use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{ant_grid::AntGrid, coord::Coord, grid_elements::ant::Team};

pub(crate) struct Runner {
    grid: AntGrid,
    rng: rand::rngs::ThreadRng,
    teams: Vec<Team>,
}
impl Runner {
    pub fn new(cols: usize, rows: usize) -> Self {
        Runner {
            grid: AntGrid::new(rows, cols),
            rng: thread_rng(),
            teams: Vec::new(),
        }
    }
    fn rand_coord(&mut self) -> Coord {
        let x = self.rng.gen_range(0..self.grid.cols);
        let y = self.rng.gen_range(0..self.grid.rows);
        Coord { x, y }
    }
    fn put_team(&mut self, color: Color) {
        let team = Team {
            color,
            id: self.teams.len(),
            health: 1,
        };
        self.teams.push(team);
        let rand = self.rand_coord();
        self.grid.put_hive(rand, team);
    }
    pub fn put_teams(&mut self) {
        self.put_team(Color::Red);
        self.put_team(Color::Blue);
        self.put_team(Color::White);
    }
    pub fn put_ants(&mut self, num_ants: usize) {
        for _ in 0..num_ants {
            let rand = self.rand_coord();
            let team = self.teams.iter().choose(&mut self.rng).unwrap();
            self.grid.put_ant(rand, team)
        }
    }
    pub fn put_food(&mut self, num_food: usize) {
        for _ in 0..num_food {
            let rand = self.rand_coord();
            self.grid.put_food(rand);
        }
    }
    pub fn print(&self) {
        println!("{}", self.grid);
    }
    pub fn run(&mut self, num_rounds: usize, interval: usize) {
        for i in 0..num_rounds {
            self.grid.run_round();
            if i % interval == interval - 1 {
                self.print();
            }
        }
    }
}