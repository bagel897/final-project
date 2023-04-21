use super::Team;
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum ElementType {
    Ant,
    Dirt,
    Empty,
    Food,
    Hive,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct TeamElement {
    pub element: ElementType,
    pub team: Option<Team>,
}
