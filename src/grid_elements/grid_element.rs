use std::fmt::{Debug, Display};

use crate::{ant_grid::AntGrid, coord::Coord};

pub(crate) trait GridElement: Debug + Display {
    fn pos(&self) -> &Coord;
    fn exists(&self) -> bool;
    fn decide(&mut self, grid: &AntGrid) -> Coord;
}
