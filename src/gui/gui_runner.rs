use crate::ant_grid::AntGrid;
use crate::tui_runner::Runner;
use egui::{TextureHandle, TextureOptions};

use super::image_utils::get_image;
struct GUIrunner {
    runner: Runner,
    texture: TextureHandle,
    speed: usize,
    rows: usize,
    cols: usize,
}
impl GUIrunner {
    pub fn new(rows: usize, cols: usize, cc: &eframe::CreationContext<'_>) -> Self {
        let mut runner = Runner::new(rows, cols);
        runner.put_teams();
        let image = get_image(&runner.grid);
        let texture = cc
            .egui_ctx
            .load_texture("ants", image, TextureOptions::default());
        GUIrunner {
            runner,
            texture,
            speed: 1,
            rows,
            cols,
        }
    }
    fn reset(&mut self) {
        self.runner = Runner::new(self.rows, self.cols);
        self.runner.put_teams();
    }
}
impl eframe::App for GUIrunner {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let input = egui::RawInput::default();
        egui::SidePanel::right("Current Round Options").show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.speed, 1..=100).text("Speed"));
            ui.add(egui::Slider::new(&mut self.runner.grid.smell, 0.01..=1.0).text("Smell"));
            if ui.button("Add food").clicked() {
                self.runner.put_food(1);
            }
            if ui.button("reset").clicked() {
                self.reset();
            }
        });
        self.runner.run(self.speed, None);
        egui::Window::new("Ant Simulation").show(&ctx, |ui| {
            self.texture
                .set(get_image(&self.runner.grid), TextureOptions::default());
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| ui.image(&self.texture, self.texture.size_vec2()),
            );
        });
        ctx.request_repaint();
    }
}
pub fn run_gui(rows: usize, cols: usize) -> Result<(), eframe::Error> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.fullscreen = true;

    return eframe::run_native(
        "My egui App",
        native_options,
        Box::new(move |cc| Box::new(GUIrunner::new(rows, cols, cc))),
    );
}
