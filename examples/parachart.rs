//! Simple plot example derived from
//! [eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
//! [plotters](https://docs.rs/plotters/0.3.4/plotters/index.html#quick-start)

use eframe::egui::{self, CentralPanel, Visuals};
use egui::Key;
use egui_plotter::{Chart, MouseConfig};
use plotters::prelude::*;
use std::ops::Range;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

struct MyEguiApp {
    chart: Chart,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        // We use data to adjust the range of the chart. This can be useful for
        // line plots where the X represents time and we want to play through
        // the X, but that is not what we are using it for here
        let chart = Chart::new()
            .mouse(MouseConfig::default().enable_all())
            .data(Box::new((-3f32..3f32, -0.5f32..3f32)))
            .builder_cb(Box::new(|mut chart_builder, _t, ranges| {
                let ranges: &(Range<f32>, Range<f32>) =
                    ranges.as_ref().unwrap().downcast_ref().unwrap();

                let (x_range, y_range) = ranges;

                let mut chart = chart_builder
                    .caption("y=x^2", ("sans-serif", 50).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(x_range.to_owned(), y_range.to_owned())
                    .unwrap();

                chart.configure_mesh().draw().unwrap();

                chart
                    .draw_series(LineSeries::new(
                        (-50 * (x_range.end as i32)..=(50 * x_range.end as i32))
                            .map(|x| x as f32 / 50.0)
                            .map(|x| (x, x * x)),
                        &RED,
                    ))
                    .unwrap()
                    .label("y = x^2")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

                chart
                    .configure_series_labels()
                    .background_style(&WHITE.mix(0.8))
                    .border_style(&BLACK)
                    .draw()
                    .unwrap();
            }));

        Self { chart }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Press 1 for the range -1..1, 2 for -2..2, 3 for -3..3
            let range = ui.input(|input| {
                if input.key_down(Key::Num1) {
                    return (-1f32..1f32, -0.5f32..1f32);
                }
                if input.key_down(Key::Num2) {
                    return (-2f32..2f32, -0.5f32..2f32);
                }

                (-3f32..3f32, -0.5f32..3f32)
            });

            // Update the range by setting the data
            self.chart.set_data(Box::new(range));

            self.chart.draw(ui);
        });
    }
}
