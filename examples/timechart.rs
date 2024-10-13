//! Simple example using the pre-defined time chart type

use std::time::Duration;

use eframe::egui::{self, CentralPanel, Visuals};
use egui::{Key, Slider, TopBottomPanel};
use egui_plotter::charts::TimeData;

const DISTANCE_M: [f32; 6] = [0.0, 2.0, 2.8, 3.4, 3.8, 4.0];
const TIME_S: [f32; 6] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "TimeData Example",
        native_options,
        Box::new(|cc| Ok(Box::new(TimeDataExample::new(cc)))),
    )
    .unwrap();
}

struct TimeDataExample {
    timechart: TimeData,
}

impl TimeDataExample {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        let mut points: Vec<(f32, f32)> = Vec::with_capacity(DISTANCE_M.len());

        for (i, distance) in DISTANCE_M.iter().enumerate() {
            points.push((TIME_S[i], *distance));
        }

        let timechart = TimeData::new(&points, "meters", "Distance Over Time");

        Self { timechart }
    }
}

impl eframe::App for TimeDataExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::bottom("playmenu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                match self.timechart.is_playing() {
                    true => {
                        if ui.button("Pause").clicked() {
                            self.timechart.toggle_playback();
                        }
                    }
                    false => {
                        if ui.button("Play").clicked() {
                            self.timechart.toggle_playback();
                        }
                    }
                }

                let start_time = self.timechart.start_time();
                let current_time = self.timechart.current_time();
                let mut set_time = current_time;
                let end_time = self.timechart.end_time();
                let mut speed = self.timechart.get_playback_speed();

                let time_slider =
                    Slider::new(&mut set_time, start_time..=end_time).show_value(false);

                let speed_slider = Slider::new(&mut speed, 0.25..=4.0).logarithmic(true);

                ui.add(time_slider);
                ui.add(speed_slider);

                if set_time != current_time {
                    self.timechart.set_time(set_time);
                }

                self.timechart.set_playback_speed(speed);

                ui.label(format!("{} : {}", current_time, end_time));
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            self.timechart.draw(ui);
        });

        // If space bar is down, start playback
        if ctx.input(|input| input.key_pressed(Key::Space)) {
            self.timechart.toggle_playback();
        }

        // Limit framerate to 100fps
        std::thread::sleep(Duration::from_millis(10));
        ctx.request_repaint();
    }
}
