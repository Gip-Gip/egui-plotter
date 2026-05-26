//! Simple plot example with a texture background, derived from
//! [eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
//! [plotters](https://docs.rs/plotters/0.3.4/plotters/index.html#quick-start)
//!
//! This example requires extra dependencies like `egui_extras` with the "`all_loaders`" feature,
//! and the `image` crate with the features enabled that corrispond with the image types you want
//! to support.

use eframe::egui::{self, CentralPanel, Visuals};
use egui::{SizeHint, TextureId, TextureOptions};
use egui_plotter::{EguiBackend, BgImageSize};
use plotters::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simple Background Example",
        native_options,
        Box::new(|cc| Ok(Box::new(SimpleBGImage::new(cc)))),
    )
    .unwrap();
}

struct SimpleBGImage {
    bg_image_id: Option<TextureId>
}

impl SimpleBGImage {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Enable light mode
        let context = &cc.egui_ctx;
        context.set_visuals(Visuals::light());
        egui_extras::install_image_loaders(context);

        Self {
            bg_image_id: None
        }
    }
}

impl eframe::App for SimpleBGImage {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        if self.bg_image_id.is_none() {
            // Sizehint doesn't affect bitmap textures(png, jpeg, gif, etc.)
            let bg_image_result = ui.ctx().try_load_texture("file://images/bg_test.png", TextureOptions::NEAREST, SizeHint::Width(4000));


            if let Some(id) = bg_image_result.unwrap().texture_id() {
                self.bg_image_id = Some(id);
            }

        }

        CentralPanel::default().show_inside(ui, |ui| {
            let root = match self.bg_image_id {
                Some(bg_image_id) => EguiBackend::new(ui).bg_image(bg_image_id, BgImageSize::Fit),
                None => EguiBackend::new(ui),
            }.into_drawing_area();

            let mut chart = ChartBuilder::on(&root)
                .caption("y=x^2", ("sans-serif", 50).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                    &RED,
                ))
                .unwrap()
                .label("y = x^2")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

            chart
                .configure_series_labels()
                .border_style(BLACK)
                .draw()
                .unwrap();

            root.present().unwrap();
        });
    }
}
