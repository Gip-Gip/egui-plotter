//! Simple spiral example using the pre-defined xytime chart type

use std::{f32::consts::PI, time::Duration};

use eframe::egui::{self, CentralPanel, Visuals};
use egui::{Key, Slider, TopBottomPanel};
use egui_plotter::charts::XyTimeData;
use plotters::style::{
    full_palette::{GREY_700, GREY_900, ORANGE_50, TEAL_400},
    ShapeStyle, WHITE,
};

const SPIRAL_LEN: usize = 10;
const SPIRAL_SUB: usize = 100;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Spiral Example",
        native_options,
        Box::new(|cc| Ok(Box::new(SprialExample::new(cc)))),
    )
    .unwrap();
}

struct SprialExample {
    spiralchart: XyTimeData,
}

impl SprialExample {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::dark());

        let mut points: Vec<(f32, f32, f32)> = Vec::with_capacity(SPIRAL_LEN * SPIRAL_SUB);

        let mut scale = 1.0 / SPIRAL_SUB as f32;
        let mut rev = PI / SPIRAL_SUB as f32;

        for i in 0..SPIRAL_LEN * SPIRAL_SUB {
            points.push((
                rev.sin() * scale,
                rev.cos() * scale,
                i as f32 / SPIRAL_SUB as f32,
            ));

            scale += 1.0 / SPIRAL_SUB as f32;
            rev += PI / SPIRAL_SUB as f32;
        }

        let spiralchart = XyTimeData::new(&points, "", "", "")
            .line_style(ShapeStyle {
                color: WHITE.into(),
                filled: false,
                stroke_width: 2,
            })
            .grid_style(ShapeStyle {
                color: GREY_700.into(),
                filled: false,
                stroke_width: 2,
            })
            .subgrid_style(ShapeStyle {
                color: GREY_900.into(),
                filled: false,
                stroke_width: 1,
            })
            .axes_style(ShapeStyle {
                color: TEAL_400.into(),
                filled: false,
                stroke_width: 2,
            })
            .text_color(ORANGE_50);

        Self { spiralchart }
    }
}

impl eframe::App for SprialExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::bottom("playmenu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                match self.spiralchart.is_playing() {
                    true => {
                        if ui.button("Pause").clicked() {
                            self.spiralchart.toggle_playback();
                        }
                    }
                    false => {
                        if ui.button("Play").clicked() {
                            self.spiralchart.toggle_playback();
                        }
                    }
                }

                let start_time = self.spiralchart.start_time();
                let current_time = self.spiralchart.current_time();
                let mut set_time = current_time;
                let end_time = self.spiralchart.end_time();
                let mut speed = self.spiralchart.get_playback_speed();

                let time_slider =
                    Slider::new(&mut set_time, start_time..=end_time).show_value(false);

                let speed_slider = Slider::new(&mut speed, 0.25..=4.0).logarithmic(true);

                ui.add(time_slider);
                ui.add(speed_slider);

                if set_time != current_time {
                    self.spiralchart.set_time(set_time);
                }

                self.spiralchart.set_playback_speed(speed);

                ui.label(format!("{} : {}", current_time, end_time));
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            self.spiralchart.draw(ui);
        });

        // If space bar is down, start playback
        if ctx.input(|input| input.key_pressed(Key::Space)) {
            self.spiralchart.toggle_playback();
        }

        // Limit framerate to 100fps
        std::thread::sleep(Duration::from_millis(10));
        ctx.request_repaint();
    }
}
