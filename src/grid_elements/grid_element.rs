use std::fmt::{Debug, Display};

use crate::{ant_grid::AntGrid, coord::Coord};

use super::ant::Team;

pub(crate) trait GridElement: Debug + Display {
    fn pos(&self) -> &Coord;
    fn exists(&self) -> bool;
    fn decide(&mut self, grid: &mut AntGrid) -> Option<Coord>;
    fn team(&self) -> Option<Team> {
        None
    }
    fn is_food(&self) -> bool {
        return false;
    }
    fn attacked(&mut self, damage: usize) {}
}
