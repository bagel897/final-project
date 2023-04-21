use crate::core::{team_element::ElementType, Team};
use std::{collections::VecDeque, fmt::Display};

use colored::{Color, Colorize};
use image::Rgb;
use strum::IntoEnumIterator;

use crate::core::{
    ant_grid::AntGrid,
    signals::{Signal, SignalType},
    {Coord, Dir},
};

use super::grid_element::GridElement;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Food,
    Battle,
    Carrying,
    Dead,
    Targeted,
}
#[derive(Debug, Clone, Copy)]
struct Targeted {
    prev_state: State,
    coord: Coord,
}

#[derive(Debug)]
pub(crate) struct Ant {
    pos: Coord,
    state: State,
    team: Team,
    health: usize,
    signals: VecDeque<Signal>,
    targeted: Option<Targeted>,
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
            State::Dead => None,
            _ => Some(self.a_star_find(grid)),
        };
        if res.is_some() && self.state == State::Carrying && self.pos != res.clone().unwrap() {
            grid.put_pheremones(res.clone().unwrap(), self.pos);
        }
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
        if self.state == State::Food {
            self.state = State::Battle;
        }
    }
    fn color(&self) -> Rgb<u8> {
        return self.team.color;
    }
    fn recv_signal(&mut self, signal: Signal) {
        self.signals.push_back(signal);
    }
    fn type_elem(&self) -> ElementType {
        ElementType::Ant
    }
}
impl Display for Ant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self.state {
            State::Carrying => "c",
            State::Food => "s",
            State::Battle => "b",
            State::Dead => "d",
            State::Targeted => "t",
        };
        let color: Color = self.team.into();
        write!(f, "{}", state.color(color))
    }
}
impl Ant {
    pub fn new(pos: &Coord, team: &Team) -> Self {
        return Ant {
            pos: pos.clone(),
            state: State::Food,
            team: team.clone(),
            health: team.health,
            signals: VecDeque::new(),
            targeted: None,
        };
    }
    fn init(&mut self) {
        match self.signals.front() {
            None => return,
            Some(i) => {
                let old_state = self.state;
                let process = match i.signal_type {
                    SignalType::Carry => old_state == State::Carrying,
                    SignalType::Food => old_state == State::Food,
                    SignalType::Battle => {
                        old_state != State::Carrying && old_state != State::Targeted
                    }
                    _ => false,
                } && old_state != State::Dead;
                if !process {
                    return;
                }
                self.targeted = Some(Targeted {
                    prev_state: old_state,
                    coord: i.coord,
                });
                self.state = State::Targeted;
            }
        };
    }
    fn cleanup(&mut self, grid: &mut AntGrid) {
        for signal in self.signals.iter() {
            if signal.propogate != 0 {
                let mut new_sig = signal.clone();
                new_sig.propogate = signal.propogate - 1;
                grid.send_signal(&self.pos, new_sig, self.team_element());
            }
        }
        self.signals.clear();
    }
    fn run_action(&mut self, pos: Coord, grid: &mut AntGrid) -> bool {
        match self.state {
            State::Carrying => {
                if grid.is_hive_same_team(&pos, self.team) {
                    grid.send_signal(
                        &pos,
                        Signal {
                            coord: pos,
                            signal_type: SignalType::Deliver,
                            propogate: 0,
                        },
                        self.team_element(),
                    );
                    self.send_carry(grid, pos);
                    self.state = State::Food;
                    return true;
                }
                return false;
            }
            State::Food => {
                if self.should_battle(grid, pos) {
                    return true;
                }
                if grid.is_food(&pos) {
                    self.send_food_signal(grid, pos);
                    self.state = State::Carrying;
                    return true;
                }
                return false;
            }
            State::Battle => {
                if grid.is_enemy(&pos, &self.team) {
                    grid.send_signal(
                        &pos,
                        Signal {
                            coord: pos,
                            signal_type: SignalType::Battle,
                            propogate: 0,
                        },
                        self.team_element(),
                    );
                    grid.attack(&pos, &self.team);
                    return true;
                }
                return false;
            }
            State::Targeted => {
                assert!(self.targeted.is_some());
                let target = self.targeted.unwrap();
                if pos == target.coord {
                    self.state = target.prev_state;
                    self.targeted = None;
                    return true;
                }
                return false;
            }
            _ => false,
        }
    }
    fn a_star_find(&mut self, grid: &mut AntGrid) -> Coord {
        let mut min_val = f64::MAX;
        let mut min_cell = Option::None;
        for dir in Dir::iter() {
            let pos = match self.pos.next_cell(&dir) {
                None => continue,
                Some(i) => i,
            };
            if self.run_action(pos, grid) {
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

    fn send_carry(&mut self, grid: &mut AntGrid, pos: Coord) {
        grid.send_signal(
            &pos,
            Signal {
                coord: pos,
                signal_type: SignalType::Carry,
                propogate: 1,
            },
            self.team_element(),
        );
    }
    fn should_battle(&mut self, grid: &mut AntGrid, coord: Coord) -> bool {
        if grid.is_enemy(&coord, &self.team) {
            grid.send_signal(
                &coord,
                Signal {
                    coord,
                    signal_type: SignalType::Battle,
                    propogate: 0,
                },
                self.team_element(),
            );
            self.state = State::Battle;
            return true;
        }
        return false;
    }
    fn send_food_signal(&mut self, grid: &mut AntGrid, pos: Coord) {
        grid.send_signal(
            &pos,
            Signal {
                coord: pos,
                signal_type: SignalType::Food,
                propogate: 1,
            },
            self.team_element(),
        );
    }
    fn get_dist(&self, pos: &Coord, grid: &mut AntGrid) -> Option<f64> {
        let res = match self.state {
            State::Food => grid.distance_to_food(&pos)?,
            State::Carrying => grid.distance_to_hive(&pos, &self.team)?,
            State::Battle => grid.distance_to_enemy(&pos, &self.team)?,
            State::Targeted => self.targeted?.coord.distance(pos),
            _ => return None,
        };
        return Some(res);
    }
}
