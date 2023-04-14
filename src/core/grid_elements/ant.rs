use std::{collections::VecDeque, fmt::Display};

use colored::{Color, Colorize};
use image::Rgb;
use strum::IntoEnumIterator;

use crate::core::{
    ant_grid::AntGrid,
    coord::{Coord, Dir},
    signals::{Signal, SignalType},
};

use super::grid_element::GridElement;
#[derive(Debug)]
enum State {
    Wandering,
    Food,
    Battle,
    Carrying,
    Dead,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Team {
    pub color: Rgb<u8>,
    pub id: usize,
    pub health: usize,
}
impl Into<Color> for Team {
    fn into(self) -> Color {
        return Color::TrueColor {
            r: self.color.0[0],
            g: self.color.0[1],
            b: self.color.0[2],
        };
    }
}
#[derive(Debug)]
pub(crate) struct Ant {
    dir: Dir,
    pos: Coord,
    state: State,
    team: Team,
    health: usize,
    signals: VecDeque<Signal>,
}
impl GridElement for Ant {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn exists(&self) -> bool {
        return true;
    }

    fn decide(&mut self, grid: &mut AntGrid) -> Option<Coord> {
        self.init();
        let res = match self.state {
            State::Wandering => Some(self.wander(grid)),
            State::Food => Some(self.find_food(grid)),
            State::Battle => Some(self.battle(grid)),
            State::Carrying => Some(self.carry(grid)),
            State::Dead => None,
        };
        self.cleanup(grid);
        res
    }
    fn team(&self) -> Option<Team> {
        Some(self.team)
    }
    fn attacked(&mut self, damage: usize) {
        match self.health.checked_sub(damage) {
            None => self.state = State::Dead,
            Some(i) => self.health = i,
        }
    }
    fn color(&self) -> Rgb<u8> {
        return self.team.color;
    }
}
impl Display for Ant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self.state {
            State::Wandering => "w",
            State::Carrying => "c",
            State::Food => "s",
            State::Battle => "b",
            State::Dead => "d",
        };
        let color: Color = self.team.into();
        write!(f, "{}", state.color(color))
    }
}
impl Ant {
    pub fn new(pos: &Coord, team: &Team) -> Self {
        return Ant {
            dir: Dir::UP,
            pos: pos.clone(),
            state: State::Food,
            team: team.clone(),
            health: team.health,
            signals: VecDeque::new(),
        };
    }
    pub(self) fn next(&self) -> Option<Coord> {
        return self.pos.next_cell(&self.dir);
    }
    fn init(&mut self) {}
    fn cleanup(&mut self, grid: &mut AntGrid) {
        for signal in self.signals.iter() {
            if signal.propogate != 0 {
                let mut new_sig = signal.clone();
                new_sig.propogate = signal.propogate - 1;
                grid.send_signal(&self.pos, new_sig, self.team);
            }
        }
        self.signals.clear();
    }
    fn carry(&mut self, grid: &mut AntGrid) -> Coord {
        let mut min_val = f64::MAX;
        let mut min_cell = Option::None;
        for dir in Dir::iter() {
            let pos = match self.pos.next_cell(&dir) {
                None => continue,
                Some(i) => i,
            };
            if grid.is_hive_same_team(&pos, self.team) {
                grid.send_signal(
                    &pos,
                    Signal {
                        coord: pos,
                        signal_type: SignalType::Deliver,
                        propogate: 0,
                    },
                    self.team,
                );
                grid.send_signal(
                    &pos,
                    Signal {
                        coord: pos,
                        signal_type: SignalType::Carry,
                        propogate: 0,
                    },
                    self.team,
                );
                self.state = State::Food;
                return self.pos;
            }
            let min = match self.get_dist(&pos, grid) {
                None => continue,
                Some(i) => i,
            };
            if min < min_val && !grid.is_blocked(&pos) {
                min_val = min;
                min_cell = Some(pos);
            }
        }
        self.pos = min_cell.unwrap_or(self.pos);
        return self.pos;
    }
    fn battle(&mut self, grid: &mut AntGrid) -> Coord {
        let next = self.next();
        if next.is_some() {
            let n = next.unwrap();
            if grid.is_enemy(&n, &self.team) {
                grid.send_signal(
                    &n,
                    Signal {
                        coord: n,
                        signal_type: SignalType::Battle,
                        propogate: 0,
                    },
                    self.team,
                );
                grid.attack(&n, &self.team);
                return self.pos;
            }
        }
        self.state = State::Wandering;

        return self.pos;
    }
    fn should_battle(&mut self, grid: &mut AntGrid, dir: Dir) -> bool {
        let coord = match self.pos().next_cell(&self.dir) {
            None => return false,
            Some(i) => i,
        };
        if grid.is_enemy(&coord, &self.team) {
            grid.send_signal(
                &coord,
                Signal {
                    coord,
                    signal_type: SignalType::Battle,
                    propogate: 0,
                },
                self.team,
            );
            self.dir = dir.clone();
            self.state = State::Battle;
            return true;
        }
        return false;
    }
    fn find_food(&mut self, grid: &mut AntGrid) -> Coord {
        let mut min_val = f64::MAX;
        let mut min_cell = Option::None;
        for dir in Dir::iter() {
            let pos = match self.pos.next_cell(&dir) {
                None => continue,
                Some(i) => i,
            };
            if self.should_battle(grid, dir) {
                return self.pos;
            }
            if grid.is_food(&pos) {
                grid.send_signal(
                    &pos,
                    Signal {
                        coord: pos,
                        signal_type: SignalType::Food,
                        propogate: 0,
                    },
                    self.team,
                );
                self.state = State::Carrying;
                return self.pos;
            }
            let min = match self.get_dist(&pos, grid) {
                None => continue,
                Some(i) => i,
            };

            if min < min_val && !grid.is_blocked(&pos) {
                min_val = min;
                min_cell = Some(pos);
            }
        }
        self.pos = min_cell.unwrap_or(self.pos);
        return self.pos;
    }
    fn get_dist(&self, pos: &Coord, grid: &mut AntGrid) -> Option<f64> {
        let res = match self.state {
            State::Food => grid.distance_to_food(&pos)?,
            State::Carrying => grid.distance_to_hive(&pos, &self.team)?,
            _ => return None,
        };
        return Some(res);
    }

    fn wander(&mut self, grid: &mut AntGrid) -> Coord {
        if self.should_battle(grid, self.dir) {
            return self.pos;
        }
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