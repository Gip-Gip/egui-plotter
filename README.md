# egui-plotter
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.txt)
[![Crates.io](https://img.shields.io/crates/v/egui-plotter)](https://crates.io/crates/egui-plotter)
[![Documentation](https://docs.rs/egui-plotter/badge.svg)](https://docs.rs/egui-plotter)
[![APE](https://img.shields.io/badge/-APE-%2359118e)](https://openapeshop.org/)
## *simple to use utilties for integrating plotter into egui*

## Usage

This crate can be used by adding `egui-plotter` to the dependencies in your
project's `Cargo.toml`.

```toml
[dependencies]
egui-plotter = "0.1.0"
```

## Examples

Here's a simple plotter example being run on naitive eframe.
Derived from
[eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
[plotters](https://docs.rs/plotters/0.3.4/plotters/index.html#quick-start).

```rust
use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::EguiBackend;
use plotters::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc)))).unwrap();
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Plotter examples are used to light mode
        cc.egui_ctx.set_visuals(Visuals::light());
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let root = EguiBackend::new(ui).into_drawing_area();
            root.fill(&WHITE).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .caption("y=x^2", ("sans-serif", 50).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32).unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                    &RED,
                )).unwrap()
                .label("y = x^2")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

            chart
                .configure_series_labels()
                .background_style(&WHITE.mix(0.8))
                .border_style(&BLACK)
                .draw().unwrap();

            root.present().unwrap();
        });
    }
}
```
