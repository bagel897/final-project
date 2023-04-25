use std::{collections::VecDeque, fmt::Display};

use colored::{Color, Colorize};
use image::Rgb;
use strum::IntoEnumIterator;

use crate::core::grid_elements::state::State;
use crate::core::grid_elements::state::State::{Carrying, Food};
use crate::core::{
    ant_grid::AntGrid,
    signals::{Signal, SignalType},
    {Coord, Dir},
};
use crate::core::{team_element::ElementType, Team};

use super::grid_element::GridElement;

#[derive(Debug, Clone)]
pub(crate) struct Ant {
    pos: Coord,
    state: State,
    team: Team,
    health: usize,
    signals: VecDeque<Signal>,
    init_propagate: usize,
}

impl GridElement for Ant {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn exists(&self) -> bool {
        return true;
    }

    fn decide(&mut self, grid: &mut AntGrid) -> Coord {
        self.init();
        self.init_propagate = grid.options.propagation;
        let res = match &self.state {
            State::Dirt { prev_state } => {
                self.state = *prev_state.clone();
                self.pos
            }
            Food { pheromones } => {
                grid.put_pheromones(self.pos, *pheromones + 1, &self.team, self.state.get_bool());
                self.state = Food {
                    pheromones: pheromones + 1,
                };
                self.pick_best_pheromones(grid)
                    .unwrap_or(self.a_star_find(grid))
            }
            Carrying { pheromones } => {
                grid.put_pheromones(self.pos, *pheromones + 1, &self.team, self.state.get_bool());
                self.state = Carrying {
                    pheromones: pheromones + 1,
                };
                self.pick_best_pheromones(grid)
                    .unwrap_or(self.a_star_find(grid))
            }
            _ => self.a_star_find(grid),
        };
        self.pos = res;
        if grid.is_dirt(&self.pos) {
            match &self.state {
                State::Dirt { prev_state: _ } => (),
                _ => {
                    self.state = State::Dirt {
                        prev_state: Box::new(self.state.clone()),
                    };
                    grid.remove_dirt(&self.pos);
                }
            }
        }
        self.cleanup(grid);
        res
    }
    fn team(&self) -> Option<Team> {
        Some(self.team)
    }
    fn type_elem(&self) -> ElementType {
        ElementType::Ant
    }
    fn attacked(&mut self, damage: usize) {
        self.health = self.health.checked_sub(damage).unwrap_or(0);
        if let Food { pheromones: _ } = self.get_state() {
            self.state = State::Battle;
        }
    }
    fn color(&self) -> Rgb<u8> {
        return self.team.color;
    }
    fn recv_signal(&mut self, signal: Signal) {
        self.signals.push_back(signal);
    }
    fn is_removed(&self) -> bool {
        return self.health == 0;
    }
}

impl Display for Ant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match &self.state {
            Carrying { pheromones: _ } => "c",
            Food { pheromones: _ } => "s",
            State::Battle => "b",
            State::Targeted {
                prev_state: _,
                coord: _,
                propagated: _,
            } => "t",
            State::Dirt { prev_state: _ } => "i",
        };
        let color: Color = self.team.into();
        write!(f, "{}", state.color(color))
    }
}

impl Ant {
    fn get_state(&self) -> State {
        match &self.state {
            State::Targeted {
                prev_state,
                coord: _,
                propagated: _,
            } => *prev_state.clone(),
            _ => self.state.clone(),
        }
    }
    pub fn new(pos: &Coord, team: &Team) -> Self {
        return Ant {
            pos: pos.clone(),
            state: Food { pheromones: 0 },
            team: team.clone(),
            health: team.health,
            signals: VecDeque::new(),
            init_propagate: 0,
        };
    }
    fn init(&mut self) {
        match self.signals.iter().max_by_key(|m| m.propagate) {
            None => return,
            Some(i) => {
                let old_state = &self.state;
                if let State::Targeted {
                    prev_state: _prev_state,
                    coord: _coord,
                    propagated,
                } = &self.state
                {
                    if i.propagate <= *propagated {
                        return;
                    }
                }
                if !match &self.get_state() {
                    State::Dirt { prev_state: _ } => false,
                    Food { pheromones: _ } => {
                        i.signal_type == SignalType::Food || i.signal_type == SignalType::Battle
                    }
                    Carrying { pheromones: _ } => i.signal_type == SignalType::Carry,
                    State::Battle => {
                        i.signal_type == SignalType::Food || i.signal_type == SignalType::Battle
                    }
                    _ => false,
                } {
                    return;
                }
                self.state = State::Targeted {
                    prev_state: Box::new(old_state.clone()),
                    coord: i.coord,
                    propagated: i.propagate,
                };
            }
        };
    }
    fn cleanup(&mut self, grid: &mut AntGrid) {
        match self.signals.iter().max_by_key(|m| m.propagate) {
            None => (),
            Some(signal) => {
                if signal.propagate != 0 {
                    let mut new_sig = signal.clone();
                    new_sig.propagate = signal.propagate.checked_sub(1).unwrap();
                    new_sig.coord = self.pos;
                    grid.send_signal(&self.pos, new_sig, self.team_element());
                }
            }
        }
        self.signals.clear();
    }
    fn run_action(&mut self, pos: Coord, grid: &mut AntGrid) -> bool {
        match &self.state {
            Carrying {
                pheromones: _pheromones,
            } => {
                if grid.is_hive_same_team(&pos, self.team) {
                    grid.send_signal(
                        &pos,
                        Signal {
                            coord: pos,
                            signal_type: SignalType::Deliver,
                            propagate: 0,
                        },
                        self.team_element(),
                    );
                    self.send_carry(grid, pos);
                    self.state = Food { pheromones: 0 };
                    return true;
                }
                return false;
            }
            Food {
                pheromones: _pheromones,
            } => {
                if self.should_battle(grid, pos) {
                    return true;
                }
                if grid.is_food(&pos) {
                    self.send_food_signal(grid, pos);
                    self.state = Carrying { pheromones: 0 };
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
                            propagate: 0,
                        },
                        self.team_element(),
                    );
                    grid.attack(&pos, &self.team);
                    return true;
                }
                return false;
            }
            State::Targeted {
                prev_state,
                coord,
                propagated: _,
            } => {
                if pos == *coord {
                    self.state = *prev_state.clone();
                    return true;
                }
                return false;
            }
            _ => false,
        }
    }
    fn pick_best_pheromones(&mut self, grid: &AntGrid) -> Option<Coord> {
        return Dir::iter()
            .filter_map(|d| self.pos.next_cell(&d))
            .filter(|p| !grid.is_blocked(p))
            .map(|pos| {
                let f = grid.get_pheromones(&pos, self.team, !self.state.get_bool());
                (pos.clone(), f)
            })
            .filter(|(_, p)| {
                *p < grid.get_pheromones(self.pos(), self.team, !self.state.get_bool())
            })
            .min_by_key(|(_, p)| *p)
            .map(|(pos, _)| pos);
    }
    fn a_star_find(&mut self, grid: &mut AntGrid) -> Coord {
        let mut min_val = f64::MAX;
        let mut min_cell = None;
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
        return min_cell.unwrap_or(self.pos);
    }

    fn send_carry(&mut self, grid: &mut AntGrid, pos: Coord) {
        grid.send_signal(
            &pos,
            Signal {
                coord: pos,
                signal_type: SignalType::Carry,
                propagate: self.init_propagate,
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
                    propagate: self.init_propagate,
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
                propagate: self.init_propagate,
            },
            self.team_element(),
        );
    }
    fn get_dist(&self, pos: &Coord, grid: &mut AntGrid) -> Option<f64> {
        let res = match &self.state {
            Food {
                pheromones: _pheromones,
            } => grid.distance_to_food(&pos)?,
            Carrying {
                pheromones: _pheromones,
            } => grid.distance_to_hive(&pos, &self.team)?,
            State::Battle => grid.distance_to_enemy(&pos, &self.team)?,
            State::Targeted {
                prev_state: _,
                coord,
                propagated: _,
            } => coord.distance(pos),
            _ => return None,
        };
        return Some(res);
    }
}
