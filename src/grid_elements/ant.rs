use std::fmt::Display;

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
    fn new(pos: &Coord) -> Self {
        return Ant {
            dir: Dir::UP,
            pos: pos.clone(),
            state: State::Wandering,
        };
    }
    fn exists(&self) -> bool {
        return true;
    }

    fn decide(&mut self, grid: &AntGrid) -> Coord {
        match self.state {
            State::Wandering => self.wander(grid),
            State::Food => todo!(),
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
