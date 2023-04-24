use std::env;

use gui::gui_runner::run_gui;
use tui::run_tui;
mod core;
mod gui;
mod tui;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    return if args.len() > 0 && args.get(1).map_or(false, |f| f == "--tui") {
        run_tui();
        Ok(())
    } else {
        run_gui(500, 1000)
    }
}
