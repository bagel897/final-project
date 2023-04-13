use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::{
    ant_grid::AntGrid,
    coord::{Coord, Dir},
};

use super::grid_element::GridElement;
#[derive(Debug)]
enum State {
    Wandering,
    Food,
    Battle,
    Carrying,
}
#[derive(Debug)]
pub(crate) struct Ant {
    dir: Dir,
    pos: Coord,
    state: State,
}
impl GridElement for Ant {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn new(pos: &Coord) -> Self {
        return Ant {
            dir: Dir::UP,
            pos: pos.clone(),
            state: State::Food,
        };
    }
    fn exists(&self) -> bool {
        return true;
    }

    fn decide(&mut self, grid: &AntGrid) -> Coord {
        match self.state {
            State::Wandering => self.wander(grid),
            State::Food => self.find_food(grid),
            State::Battle => todo!(),
            State::Carrying => todo!(),
        }
    }
}
impl Display for Ant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self.state {
            State::Wandering => "w",
            State::Carrying => "c",
            State::Food => "s",
            State::Battle => "b",
        };
        write!(f, "{}", state)
    }
}
impl Ant {
    pub(self) fn next(&self) -> Option<Coord> {
        return self.pos.next_cell(&self.dir);
    }
    fn find_food(&mut self, grid: &AntGrid) -> Coord {
        let mut min_val = f64::MAX;
        let mut min_cell = Option::None;
        for dir in Dir::iter() {
            let min = match self.get_dist(grid, &dir) {
                None => continue,
                Some(i) => i,
            };
            if min.1 < min_val {
                min_val = min.1;
                min_cell = Some(min.0);
            }
        }
        self.pos = min_cell.unwrap_or(self.pos);
        return self.pos;
    }
    fn get_dist(&self, grid: &AntGrid, dir: &Dir) -> Option<(Coord, f64)> {
        let pos = self.pos.next_cell(&dir)?;
        let res = grid.distance_to_food(&pos)?;
        return Some((pos, res));
    }

    fn wander(&mut self, grid: &AntGrid) -> Coord {
        match self.next() {
            None => {
                self.dir = self.dir.turn();
                return self.pos;
            }
            Some(i) => {
                if grid.is_blocked(&i) {
                    self.dir = self.dir.turn();
                    return self.pos;
                }
                self.pos = i;
                return i;
            }
        }
    }
}
