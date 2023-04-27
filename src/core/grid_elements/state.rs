use crate::core::Coord;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum State {
    Food {
        pheromones: usize,
    },
    Battle {
        rage: usize,
    },
    Carrying {
        pheromones: usize,
    },
    Targeted {
        prev_state: Box<State>,
        coord: Coord,
        propagated: usize,
    },
    Dirt {
        prev_state: Box<State>,
    },
}

impl State {
    pub fn get_bool(&self) -> bool {
        match self {
            State::Food { .. } => true,
            State::Carrying { .. } => false,
            _ => panic!(),
        }
    }
}
