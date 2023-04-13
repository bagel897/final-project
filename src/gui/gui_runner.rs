use crate::ant_grid::AntGrid;
use crate::tui_runner::Runner;
use egui::{ColorImage, TextureHandle, TextureOptions};

use super::image_utils::get_image;
struct GUIrunner {
    runner: Runner,
    texture: TextureHandle,
    speed: usize,
}
impl GUIrunner {
    pub fn new(rows: usize, cols: usize, cc: &eframe::CreationContext<'_>) -> Self {
        let mut runner = Runner::new(rows, cols);
        runner.put_teams();
        runner.put_food(10);
        let image = get_image(&runner.grid);
        let texture = cc
            .egui_ctx
            .load_texture("ants", image, TextureOptions::default());
        GUIrunner {
            runner,
            texture,
            speed: 1,
        }
    }
}
impl eframe::App for GUIrunner {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let input = egui::RawInput::default();
        egui::Window::new("My Window").show(&ctx, |ui| {
            self.runner.run(self.speed, None);
            self.texture
                .set(get_image(&self.runner.grid), TextureOptions::default());
            ui.image(&self.texture, self.texture.size_vec2());
            ui.add(egui::Slider::new(&mut self.speed, 1..=100));
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
