use gui::gui_runner::run_gui;

mod core;
mod gui;
mod tui;

fn main() {
    run_gui(500, 1000);
}
