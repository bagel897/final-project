use gui::gui_runner::run_gui;

use crate::tui_runner::Runner;
mod ant_grid;
mod coord;
mod grid_elements;
mod gui;
mod tui_runner;

fn run_tui() {
    let num_ants = 0;
    let num_food = 4;
    let cols = 40;
    let rows = 20;
    let num_rounds = 100;
    let interval = 5;
    let mut runner = Runner::new(rows, cols);
    runner.put_teams();
    runner.put_ants(num_ants);
    runner.put_food(num_food);
    runner.print();
    runner.run(num_rounds, Some(interval));
}
fn main() {
    run_gui(500, 1000);
}
