use super::coord::Coord;

#[derive(Clone, Copy)]
pub(crate) enum  SignalType {
    Food,
    Carry,
    Battle,
    Deliver,
}
#[derive(Clone, Copy)]
pub(crate) struct Signal {
    pub coord: Coord,
    pub signal_type: SignalType,
    pub propogate: usize,
}
