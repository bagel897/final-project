use std::fmt::Display;

use colored::Colorize;
use strum::IntoEnumIterator;

use crate::coord::{Coord, Dir};

use super::{ant::Team, grid_element::GridElement};
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Hive {
    pos: Coord,
    team: Team,
    health: usize,
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
    fn decide(&mut self, grid: &mut crate::ant_grid::AntGrid) -> Option<Coord> {
        if self.health == 0 {
            return None;
        }
        for dir in Dir::iter() {
            let next = self.pos.next_cell(&dir);
            if next.is_some() {
                let n = next.unwrap();
                if !grid.is_blocked(&n) {
                    grid.put_ant(n, &self.team);
                    break;
                }
            }
        }
        return Some(self.pos);
    }
    fn is_hive(&self) -> bool {
        return true;
    }
}
impl Hive {
    pub fn new(pos: Coord, team: Team) -> Self {
        Hive {
            pos,
            team,
            health: team.health,
        }
    }
}
impl Display for Hive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "h".color(self.team.color).bold())
    }
}
