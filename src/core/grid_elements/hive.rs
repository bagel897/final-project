use std::fmt::Display;

use colored::{Color, Colorize};
use image::Rgb;
use strum::IntoEnumIterator;

use crate::core::{signals::SignalType, team_element::ElementType, Ant, AntGrid, Coord, Dir, Team};

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
    fn exists(&self) -> bool {
        return true;
    }
    fn decide(&mut self, grid: &mut AntGrid) -> Coord {
        for dir in Dir::iter() {
            let next = self.pos.next_cell(&dir);
            if next.is_some() {
                let n = next.unwrap();
                if !grid.is_blocked(&n) {
                    if self.food > 0 {
                        grid.put(Ant::new(&n, &self.team));
                        self.food -= 1;
                    }
                    break;
                }
            }
        }
        self.pos
    }
    fn team(&self) -> Option<Team> {
        return Some(self.team);
    }
    fn type_elem(&self) -> ElementType {
        ElementType::Hive
    }
    fn attacked(&mut self, damage: usize) {
        self.health = self.health.checked_sub(damage).unwrap_or(0);
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
    fn is_removed(&self) -> bool {
        return self.health == 0;
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
