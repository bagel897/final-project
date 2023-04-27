use std::time::Instant;

use eframe::Renderer;
use egui::{Frame, Image, Pos2, TextureHandle, TextureOptions, Vec2};
use puffin;
use puffin_egui;

use crate::core::Options;
use crate::core::{BaseRunner, Coord, Dirt, Food, Hive, Runner, Team};

#[derive(PartialEq, Eq, Clone, Copy)]
enum SelectionMode {
    DIRT,
    HIVE,
    FOOD,
}

struct Timer {
    frames: usize,
    start: Instant,
}

impl Timer {
    fn new(frames: usize) -> Self {
        Timer {
            frames,
            start: Instant::now(),
        }
    }
    fn tick(&mut self, num_frames: usize) {
        self.frames += num_frames;
    }
    fn set(&mut self, num_frames: usize) {
        self.frames = num_frames;
    }
    fn fps(&self) -> f64 {
        let time = self.start.elapsed().as_secs();
        return (self.frames as f64) / (time as f64);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct AddMode {
    team: Option<Team>,
    selection_mode: SelectionMode,
}

const FOOD_MODE: AddMode = AddMode {
    team: None,
    selection_mode: SelectionMode::FOOD,
};
const DIRT_MODE: AddMode = AddMode {
    team: None,
    selection_mode: SelectionMode::DIRT,
};

type RunnerMode = BaseRunner;

struct GUIrunner {
    runner: RunnerMode,
    texture: TextureHandle,
    rows: usize,
    cols: usize,
    timer: Timer,
    add_mode: AddMode,
    profile: bool,
    options: Options,
}

impl GUIrunner {
    pub fn new(rows: usize, cols: usize, cc: &eframe::CreationContext<'_>) -> Self {
        let options = Options::default();
        let mut runner = RunnerMode::new(rows, cols, options);
        let frames = runner.export().frames();
        let image = runner.export().to_image();
        let texture = cc
            .egui_ctx
            .load_texture("ants", image, TextureOptions::default());
        GUIrunner {
            runner,
            texture,
            rows,
            cols,
            timer: Timer::new(frames),
            add_mode: FOOD_MODE,
            profile: false,
            options,
        }
    }
    fn reset(&mut self) {
        self.runner.set_opts(self.options);
        self.runner.reset();
        self.timer_reset();
    }
    fn timer_reset(&mut self) {
        self.timer = Timer::new(self.runner.export().frames());
    }
    fn add(&mut self, rect: Pos2, _drag: Vec2) {
        let y = rect.y as usize;
        let x = rect.x as usize;
        let c = Coord { x, y };
        match self.add_mode.selection_mode {
            SelectionMode::HIVE => self.runner.put(Hive::new(
                c,
                self.add_mode.team.unwrap(),
                self.options.starting_food,
            )),
            SelectionMode::FOOD => self.runner.put(Food::new(&c)),
            SelectionMode::DIRT => {
                self.runner.put(Dirt::new(&c));
                // let mut bounds = [0, drag.x.round() as u32];
                // bounds.sort();
                // println!("{:?}", bounds);
                // for drag_x in bounds[0] - 1..bounds[1] + 1 {
                //     println!("drag_x{:?}", drag_x);
                //     let unround_y_floor = ((drag_x - 1) as f32 / drag.x) * drag.y;
                //     let unround_y_ceil = ((drag_x + 1) as f32 / drag.x) * drag.y;
                //     let mut bounds = [
                //         unround_y_floor.round() as usize,
                //         unround_y_ceil.round() as usize,
                //     ];
                //     bounds.sort();
                //     println!("{:?}", bounds);
                //     for drag_y in bounds[0]..bounds[1] {
                //         let c = Coord {
                //             x: (drag_x + x as u32) as usize,
                //             y: drag_y + y,
                //         };
                //         self.runner.grid.put_dirt(c);
                //     }
                // }
            }
        }
    }
}

impl eframe::App for GUIrunner {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.runner.run();
        let export = self.runner.export();
        puffin::GlobalProfiler::lock().new_frame();
        if self.profile {
            puffin_egui::profiler_window(ctx);
        }
        let _input = egui::RawInput::default();
        egui::SidePanel::right("Current Round Options").show(&ctx, |ui| {
            if ui
                .add(egui::Slider::new(&mut self.options.speed, 1..=100).text("Rounds/frame"))
                .changed()
            {
                self.timer_reset();
            }
            ui.add(egui::Slider::new(&mut self.options.smell, 0.01..=1.0).text("Smell offset"));
            ui.add(
                egui::Slider::new(&mut self.options.signal_radius, 0.0..=1000.0)
                    .text("Signal Radius"),
            );
            ui.add(
                egui::Slider::new(&mut self.options.decay, 0..=10000)
                    .text("Pheromone decay (rounds)"),
            );
            ui.add(
                egui::Slider::new(&mut self.options.starting_food, 1..=100).text("Starting Food"),
            );
            ui.add(egui::Slider::new(&mut self.options.propagation, 0..=10).text("propagation"));

            // if ui.button("Add food (random)").clicked() {
            //     self.runner.put_food(1);
            // }
            if ui.button("Reset grid").clicked() {
                self.reset();
            }
            ui.checkbox(&mut self.profile, "Show profiler");
            ui.radio_value(&mut self.add_mode, FOOD_MODE, "Add Food");
            ui.radio_value(&mut self.add_mode, DIRT_MODE, "Add Dirt");
            for team in export.teams().iter() {
                ui.radio_value(
                    &mut self.add_mode,
                    AddMode {
                        team: Some(team.clone()),
                        selection_mode: SelectionMode::HIVE,
                    },
                    format!("Add Hive {:?}", team.name),
                );
            }

            ui.add(egui::Label::new(format!(
                "Rounds Per Second: {}",
                self.timer.fps()
            )))
        });
        self.runner.set_opts(self.options);
        self.timer.set(export.frames());
        egui::Window::new("Ant Simulation")
            .collapsible(false)
            .title_bar(false)
            .movable(false)
            .constrain(true)
            .frame(Frame::none())
            .show(&ctx, |ui| {
                self.texture
                    .set(export.to_image(), TextureOptions::default());
                let rect = ui.available_size();
                let y = rect.y as usize;
                let x = rect.x as usize;
                if y != self.rows || x != self.cols {
                    self.rows = y;
                    self.cols = x;
                    self.reset();
                }
                let image = Image::new(&self.texture, self.texture.size_vec2())
                    .sense(egui::Sense::click_and_drag());
                let response = ui.add(image);
                response
                    .interact_pointer_pos()
                    .map(|p| self.add(p, response.drag_delta()));
            });
        ctx.request_repaint();
    }
}

pub fn run_gui(rows: usize, cols: usize) -> Result<(), eframe::Error> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.fullscreen = true;
    native_options.renderer = Renderer::Wgpu;
    puffin::set_scopes_on(true);
    return eframe::run_native(
        "My egui App",
        native_options,
        Box::new(move |cc| Box::new(GUIrunner::new(rows, cols, cc))),
    );
}
