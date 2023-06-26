# egui-plotter
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.txt)
[![Crates.io](https://img.shields.io/crates/v/egui-plotter)](https://crates.io/crates/egui-plotter)
[![Documentation](https://docs.rs/egui-plotter/badge.svg)](https://docs.rs/egui-plotter)
[![APE](https://img.shields.io/badge/-APE-%2359118e)](https://openapeshop.org/)
## *simple to use utilties for integrating plotter into egui*

[![3d Graph Live Demo](https://github.com/Gip-Gip/egui-plotter/blob/91a86d3dfcd8f4f1207284030edcb637b2edc973/images/3d.gif?raw=true)](https://github.com/Gip-Gip/egui-plotter/blob/main/examples/3d.rs)
[![spiral live demo](https://github.com/gip-gip/egui-plotter/blob/945886c8f6883b76955df3bce6e8bf2541cc5571/images/spiral.gif?raw=true)](https://github.com/gip-gip/egui-plotter/blob/main/examples/spiral.rs)

## usage

this crate can be used by adding `egui-plotter` to the dependencies in your
project's `cargo.toml`.

```toml
[dependencies]
egui-plotter = "0.3.0"
```

**it is also heavily recommended you disable feathering in your egui context,
as not only does it slow things down but it causes artifacts with certain plots.**

see line 24 example below to see how to disable feathering.

## examples

here's a simple plotter example being run on native eframe.
derived from
[eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
[plotters](https://docs.rs/plotters/0.3.4/plotters/index.html#quick-start).

```rust
use eframe::egui::{self, centralpanel, visuals};
use egui_plotter::eguibackend;
use plotters::prelude::*;

fn main() {
    let native_options = eframe::nativeoptions::default();
    eframe::run_native(
        "simple example",
        native_options,
        box::new(|cc| box::new(simple::new(cc))),
    )
    .unwrap();
}

struct simple;

impl simple {
    fn new(cc: &eframe::creationcontext<'_>) -> self {
        // disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // also enable light mode
        context.set_visuals(visuals::light());

        self
    }
}

impl eframe::app for simple {
    fn update(&mut self, ctx: &egui::context, _frame: &mut eframe::frame) {
        centralpanel::default().show(ctx, |ui| {
            let root = eguibackend::new(ui).into_drawing_area();
            root.fill(&white).unwrap();
            let mut chart = chartbuilder::on(&root)
                .caption("y=x^2", ("sans-serif", 50).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(lineseries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                    &red,
                ))
                .unwrap()
                .label("y = x^2")
                .legend(|(x, y)| pathelement::new(vec![(x, y), (x + 20, y)], &red));

            chart
                .configure_series_labels()
                .background_style(&white.mix(0.8))
                .border_style(&black)
                .draw()
                .unwrap();

            root.present().unwrap();
        });
    }
}
```

### charts

alternatively, the above example can be made with a chart type to allow easy
user interactivity with your plotter charts. you can either make your own chart or
use a prebuilt chart type included in the `charts` module.

```rust
use eframe::egui::{self, centralpanel, key, visuals};
use egui_plotter::{chart, mouseconfig};
use plotters::prelude::*;
use std::ops::range;

fn main() {
    let native_options = eframe::nativeoptions::default();
    eframe::run_native(
        "parachart example",
        native_options,
        box::new(|cc| box::new(parachart::new(cc))),
    )
    .unwrap();
}

struct parachart {
    chart: chart<(range<f32>, range<f32>)>,
}

impl parachart {
    fn new(cc: &eframe::creationcontext<'_>) -> self {
        // disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // also enable light mode
        context.set_visuals(visuals::light());

        // we use data to adjust the range of the chart. this can be useful for
        // line plots where the x represents time and we want to play through
        // the x, but that is not what we are using it for here
        let chart = chart::new((-3f32..3f32, -0.5f32..3f32))
            .mouse(mouseconfig::enabled())
            .builder_cb(box::new(|area, _t, ranges| {
                // build a chart like you would in any other plotter chart.
                // the drawing area and ranges are provided by the callback,
                // but otherwise everything else is the same.

                let (x_range, y_range) = ranges;

                let mut chart = chartbuilder::on(area)
                    .caption("y=x^2", ("sans-serif", 50).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(x_range.to_owned(), y_range.to_owned())
                    .unwrap();

                chart.configure_mesh().draw().unwrap();

                chart
                    .draw_series(lineseries::new(
                        (-50 * (x_range.end as i32)..=(50 * x_range.end as i32))
                            .map(|x| x as f32 / 50.0)
                            .map(|x| (x, x * x)),
                        &red,
                    ))
                    .unwrap()
                    .label("y = x^2")
                    .legend(|(x, y)| pathelement::new(vec![(x, y), (x + 20, y)], red));

                chart
                    .configure_series_labels()
                    .background_style(white.mix(0.8))
                    .border_style(black)
                    .draw()
                    .unwrap();
            }));

        self { chart }
    }
}

impl eframe::App for ParaChart {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Press 1 for the range -1..1, 2 for -2..2, 3 for -3..3
            ui.input(|input| {
                if input.key_down(Key::Num1) {
                    *self.chart
                        .get_data_mut() = (-1f32..1f32, -0.5f32..1f32);
                }
                if input.key_down(Key::Num2) {
                    *self.chart
                        .get_data_mut() = (-2f32..2f32, -0.5f32..2f32);
                }

                if input.key_down(Key::Num3) {
                    *self.chart
                        .get_data_mut() = (-3f32..3f32, -0.5f32..3f32);
                }
            });

            self.chart.draw(ui);
        });
    }
}
```
