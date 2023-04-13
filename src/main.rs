use crate::tui_runner::Runner;
mod ant_grid;
mod coord;
mod grid_elements;
mod gui;
mod tui_runner;

fn main() {
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
    runner.run(num_rounds, interval);
}
