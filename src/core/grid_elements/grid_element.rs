use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use image::Rgb;

use crate::core::{
    signals::Signal,
    team_element::{ElementType, TeamElement},
    AntGrid, Coord, Team,
};

pub(crate) trait GridElement: Debug + Display + Send {
    fn pos(&self) -> &Coord;
    fn exists(&self) -> bool;
    fn decide(&mut self, grid: &mut AntGrid) -> Coord;
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
    fn is_removed(&self) -> bool {
        return false;
    }
}
impl<T: GridElement + 'static> IntoHelper for T {
    fn to_refcell(self: Box<Self>) -> Rc<RefCell<dyn GridElement>> {
        Rc::new(RefCell::new(*self))
    }
}
impl Into<Rc<RefCell<dyn GridElement>>> for Box<dyn IntoHelper> {
    fn into(self) -> Rc<RefCell<dyn GridElement>> {
        self.to_refcell()
    }
}
pub(crate) trait IntoHelper: Send {
    fn to_refcell(self: Box<Self>) -> Rc<RefCell<dyn GridElement>>;
}
