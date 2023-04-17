use std::time::Instant;

use crate::core::runner::Runner;
use egui::{TextureHandle, TextureOptions};

use super::image_utils::get_image;
struct Timer {
    frames: usize,
    start: Instant,
}
impl Timer {
    fn new() -> Self {
        Timer {
            frames: 0,
            start: Instant::now(),
        }
    }
    fn tick(&mut self, num_frames: usize) {
        self.frames += num_frames;
    }
    fn fps(&self) -> f64 {
        let time = self.start.elapsed().as_secs();
        return (self.frames as f64) / (time as f64);
    }
}
struct GUIrunner {
    runner: Runner,
    texture: TextureHandle,
    speed: usize,
    rows: usize,
    cols: usize,
    timer: Timer,
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
            timer: Timer::new(),
        }
    }
    fn reset(&mut self) {
        self.runner = Runner::new(self.rows, self.cols);
        self.runner.put_teams();
        self.timer_reset();
    }
    fn timer_reset(&mut self) {
        self.timer = Timer::new();
    }
}
impl eframe::App for GUIrunner {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let input = egui::RawInput::default();
        egui::SidePanel::right("Current Round Options").show(&ctx, |ui| {
            if ui
                .add(egui::Slider::new(&mut self.speed, 1..=100).text("Speed"))
                .changed()
            {
                self.timer_reset();
            }
            ui.add(egui::Slider::new(&mut self.runner.grid.smell, 0.01..=1.0).text("Smell"));
            if ui.button("Add food").clicked() {
                self.runner.put_food(1);
            }
            if ui.button("reset").clicked() {
                self.reset();
            }
            ui.add(egui::Label::new(format!("FPS: {}", self.timer.fps())))
        });
        self.runner.run(self.speed, None);
        self.timer.tick(self.speed);
        egui::Window::new("Ant Simulation").show(&ctx, |ui| {
            self.texture
                .set(get_image(&self.runner.grid), TextureOptions::default());
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let rect = ui.available_size();
                    let y = rect.y as usize;
                    let x = rect.x as usize;
                    if y != self.rows || x != self.cols {
                        self.rows = y;
                        self.cols = x;
                        self.reset();
                    }
                    ui.image(&self.texture, self.texture.size_vec2());
                },
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
