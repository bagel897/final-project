use std::fmt::{Debug, Display};

use image::Rgb;

use crate::core::{
    signals::Signal,
    team_element::{ElementType, TeamElement},
    AntGrid, Coord, Team,
};

pub(crate) trait GridElement: Debug + Display {
    fn pos(&self) -> &Coord;
    fn exists(&self) -> bool;
    fn decide(&mut self, grid: &mut AntGrid) -> Option<Coord>;
    fn team(&self) -> Option<Team> {
        None
    }
    fn type_elem(&self) -> ElementType;
    fn team_element(&self) -> TeamElement {
        TeamElement {
            element: self.type_elem(),
            team: self.team(),
        }
    }
    fn attacked(&mut self, _damage: usize) {}
    fn color(&self) -> Rgb<u8>;
    fn recv_signal(&mut self, _signal: Signal) {}
}
