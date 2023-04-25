#[derive(Clone, Copy)]
pub(crate) struct Options {
    pub smell: f64,
    pub starting_food: usize,
    pub signal_radius: f64,
    pub dirt_penalty: f64,
    pub speed: usize,
    pub propagation: usize,
}

impl Default for Options {
    fn default() -> Self {
        return Options {
            smell: 0.5,
            starting_food: 10,
            signal_radius: 2.0,
            speed: 20,
            propagation: 3,
            dirt_penalty: 1.2,
        };
    }
}
