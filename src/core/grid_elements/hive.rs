use std::fmt::Display;

use colored::{Color, Colorize};
use image::Rgb;
use strum::IntoEnumIterator;

use crate::core::{signals::SignalType, team_element::ElementType, AntGrid, Coord, Dir, Team};

use super::grid_element::GridElement;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Hive {
    pos: Coord,
    team: Team,
    health: usize,
    food: usize,
}
impl GridElement for Hive {
    fn pos(&self) -> &Coord {
        return &self.pos;
    }
    fn team(&self) -> Option<Team> {
        return Some(self.team);
    }
    fn exists(&self) -> bool {
        return true;
    }
    fn attacked(&mut self, damage: usize) {
        self.health = self.health.checked_sub(damage).unwrap_or(0);
    }
    fn decide(&mut self, grid: &mut AntGrid) -> Option<Coord> {
        if self.health == 0 {
            return None;
        }
        for dir in Dir::iter() {
            let next = self.pos.next_cell(&dir);
            if next.is_some() {
                let n = next.unwrap();
                if !grid.is_blocked(&n) {
                    if self.food > 0 {
                        grid.put_ant(n, &self.team);
                        self.food -= 1;
                    }
                    break;
                }
            }
        }
        return Some(self.pos);
    }
    fn color(&self) -> Rgb<u8> {
        return self.team.color;
    }
    fn recv_signal(&mut self, signal: crate::core::signals::Signal) {
        match signal.signal_type {
            SignalType::Deliver => self.food += 1,
            _ => {}
        };
    }
    fn type_elem(&self) -> ElementType {
        ElementType::Hive
    }
}
impl Hive {
    pub fn new(pos: Coord, team: Team, food: usize) -> Self {
        Hive {
            pos,
            team,
            health: team.health,
            food,
        }
    }
}
impl Display for Hive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color: Color = self.team.into();
        write!(f, "{}", "h".color(color).bold())
    }
}
