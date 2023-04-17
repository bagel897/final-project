use std::time::Instant;

use crate::core::{Coord, Runner, Team};
use egui::{Frame, Image, Pos2, TextureHandle, TextureOptions};

#[derive(PartialEq)]
enum SelectionMode {
    DIRT,
    HIVE,
    FOOD,
}
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
struct GUIrunner<'a> {
    runner: Runner,
    texture: TextureHandle,
    speed: usize,
    rows: usize,
    cols: usize,
    timer: Timer,
    selection_mode: SelectionMode,
    team: Option<&'a Team>,
}
impl<'a> GUIrunner<'a> {
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
            selection_mode: SelectionMode::FOOD,
            team: None,
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
    fn add(&mut self, rect: Pos2) {
        let y = rect.y as usize;
        let x = rect.x as usize;
        let c = Coord { x, y };
        match self.selection_mode {
            SelectionMode::HIVE => todo!(),
            SelectionMode::FOOD => self.runner.grid.put_food(c),
            SelectionMode::DIRT => self.runner.grid.put_dirt(c),
        }
    }
}
impl<'a> eframe::App for GUIrunner<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let input = egui::RawInput::default();
        egui::SidePanel::right("Current Round Options").show(&ctx, |ui| {
            if ui
                .add(egui::Slider::new(&mut self.speed, 1..=100).text("Rounds/frame"))
                .changed()
            {
                self.timer_reset();
            }
            ui.add(egui::Slider::new(&mut self.runner.grid.smell, 0.01..=1.0).text("Smell offset"));
            ui.add(
                egui::Slider::new(&mut self.runner.grid.signal_radius, 0.0..=1000.0)
                    .text("Signal Radius"),
            );
            if ui.button("Add food (random)").clicked() {
                self.runner.put_food(1);
            }
            if ui.button("Reset grid").clicked() {
                self.reset();
            }
            ui.radio_value(&mut self.selection_mode, SelectionMode::FOOD, "Add Food");
            ui.radio_value(&mut self.selection_mode, SelectionMode::DIRT, "Add Dirt");
            ui.radio_value(&mut self.selection_mode, SelectionMode::HIVE, "Add Hive");

            ui.add(egui::Label::new(format!(
                "Rounds Per Second: {}",
                self.timer.fps()
            )))
        });
        self.timer.tick(self.runner.run_dynamic(self.speed));
        egui::Window::new("Ant Simulation")
            .collapsible(false)
            .title_bar(false)
            .frame(Frame::none())
            .show(&ctx, |ui| {
                self.texture
                    .set(get_image(&self.runner.grid), TextureOptions::default());
                let rect = ui.available_size();
                let y = rect.y as usize;
                let x = rect.x as usize;
                if y != self.rows || x != self.cols {
                    self.rows = y;
                    self.cols = x;
                    self.reset();
                }
                let image =
                    Image::new(&self.texture, self.texture.size_vec2()).sense(egui::Sense::click());
                let response = ui.add(image);
                response.interact_pointer_pos().map(|p| self.add(p));
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
