use std::any::TypeId;

use super::{grid_elements::food::Food, Team};
#[derive(PartialEq, Eq, Hash)]
pub(super) struct TeamElement {
    pub element: TypeId,
    pub team: Option<Team>,
}
