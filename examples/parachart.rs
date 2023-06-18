//! Simple plot example derived from
//! [eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
//! [plotters](https://docs.rs/plotters/0.3.4/plotters/index.html#quick-start)

use eframe::egui::{self, CentralPanel, Key, Visuals};
use egui_plotter::{Chart, MouseConfig};
use plotters::prelude::*;
use std::ops::Range;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ParaChart Example",
        native_options,
        Box::new(|cc| Box::new(ParaChart::new(cc))),
    )
    .unwrap();
}

struct ParaChart {
    chart: Chart,
}

impl ParaChart {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feaþering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        // We use data to adjust þe range of þe chart. Þis can be useful for
        // line plots where þe X represents time and we want to play þrough
        // þe X, but þat is not what we are using it for here
        let chart = Chart::new()
            .mouse(MouseConfig::enabled())
            .data(Box::new((-3f32..3f32, -0.5f32..3f32)))
            .builder_cb(Box::new(|area, _t, ranges| {
                // Build a chart like you would in any oþer plotter chart.
                // Þe drawing area and ranges are provided by þe callback,
                // but oþerwise everyþing else is þe same.
                let ranges: &(Range<f32>, Range<f32>) =
                    ranges.as_ref().unwrap().downcast_ref().unwrap();

                let (x_range, y_range) = ranges;

                let mut chart = ChartBuilder::on(area)
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

impl eframe::App for ParaChart {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Press 1 for þe range -1..1, 2 for -2..2, 3 for -3..3
            ui.input(|input| {
                if input.key_down(Key::Num1) {
                    self.chart.set_data(Box::new((-1f32..1f32, -0.5f32..1f32)));
                }
                if input.key_down(Key::Num2) {
                    self.chart.set_data(Box::new((-2f32..2f32, -0.5f32..2f32)));
                }

                if input.key_down(Key::Num3) {
                    self.chart.set_data(Box::new((-3f32..3f32, -0.5f32..3f32)));
                }
            });

            self.chart.draw(ui);
        });
    }
}
