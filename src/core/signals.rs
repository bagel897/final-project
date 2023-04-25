use super::Coord;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SignalType {
    Food,
    Carry,
    Battle,
    Deliver,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Signal {
    pub coord: Coord,
    pub signal_type: SignalType,
    pub propagate: usize,
}
