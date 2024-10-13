//! Simple 3d plot example derived from
//! [eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
//! [plotters](https://github.com/plotters-rs/plotters/blob/master/plotters/examples/3d-plot.rs)

use std::time::Duration;

use egui::{self, CentralPanel, Visuals};
use egui_plotter::EguiBackend;
use plotters::prelude::*;
const MOVE_SCALE: f32 = 0.01;
const SCROLL_SCALE: f32 = 0.001;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "3d Example",
        native_options,
        Box::new(|cc| Ok(Box::new(ThreeD::new(cc)))),
    )
    .unwrap();
}

struct ThreeD {
    chart_pitch: f32,
    chart_yaw: f32,
    chart_scale: f32,
    chart_pitch_vel: f32,
    chart_yaw_vel: f32,
}

impl ThreeD {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        Self {
            chart_pitch: 0.3,
            chart_yaw: 0.9,
            chart_scale: 0.9,
            chart_pitch_vel: 0.0,
            chart_yaw_vel: 0.0,
        }
    }
}

impl eframe::App for ThreeD {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // First, get mouse data
            let (pitch_delta, yaw_delta, scale_delta) = ui.input(|input| {
                let pointer = &input.pointer;
                let delta = pointer.delta();

                let (pitch_delta, yaw_delta) = match pointer.primary_down() {
                    true => (delta.y * MOVE_SCALE, -delta.x * MOVE_SCALE),
                    false => (self.chart_pitch_vel, self.chart_yaw_vel),
                };

                let scale_delta = input.raw_scroll_delta.y * SCROLL_SCALE;

                (pitch_delta, yaw_delta, scale_delta)
            });

            self.chart_pitch_vel = pitch_delta;
            self.chart_yaw_vel = yaw_delta;

            self.chart_pitch += self.chart_pitch_vel;
            self.chart_yaw += self.chart_yaw_vel;
            self.chart_scale += scale_delta;

            // Next plot everything
            let root = EguiBackend::new(ui).into_drawing_area();

            root.fill(&WHITE).unwrap();

            let x_axis = (-3.0..3.0).step(0.1);
            let z_axis = (-3.0..3.0).step(0.1);

            let mut chart = ChartBuilder::on(&root)
                .caption(format!("3D Plot Test"), (FontFamily::SansSerif, 20))
                .build_cartesian_3d(x_axis, -3.0..3.0, z_axis)
                .unwrap();

            chart.with_projection(|mut pb| {
                pb.yaw = self.chart_yaw as f64;
                pb.pitch = self.chart_pitch as f64;
                pb.scale = self.chart_scale as f64;
                pb.into_matrix()
            });

            chart
                .configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .max_light_lines(3)
                .draw()
                .unwrap();

            chart
                .draw_series(
                    SurfaceSeries::xoz(
                        (-30..30).map(|f| f as f64 / 10.0),
                        (-30..30).map(|f| f as f64 / 10.0),
                        |x, z| (x * x + z * z).cos(),
                    )
                    .style(BLUE.mix(0.2).filled()),
                )
                .unwrap()
                .label("Surface")
                .legend(|(x, y)| {
                    Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
                });

            chart
                .draw_series(LineSeries::new(
                    (-100..100)
                        .map(|y| y as f64 / 40.0)
                        .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                    &BLACK,
                ))
                .unwrap()
                .label("Line")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

            chart
                .configure_series_labels()
                .border_style(BLACK)
                .draw()
                .unwrap();

            root.present().unwrap();
        });

        // Limit framerate to 100fps
        std::thread::sleep(Duration::from_millis(10));
        ctx.request_repaint();
    }
}
