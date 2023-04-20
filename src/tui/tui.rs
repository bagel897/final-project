use crate::core::runner::Runner;
pub(crate) fn run_tui() {
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
