use crate::core::{ant_grid::Options, BaseRunner, Runner};
pub(crate) fn run_tui() {
    let num_ants = 0;
    let num_food = 4;
    let cols = 40;
    let rows = 20;
    let num_rounds = 100;
    let interval = 5;
    let mut runner = BaseRunner::new(rows, cols, Options::default());
    todo!();
    // runner.put_ants(num_ants);
    // runner.put_food(num_food);
    // runner.print();
    // runner.run(num_rounds, Some(interval));
}
