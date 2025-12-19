//! Simple 3d plot example derived from
//! [eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
//! [plotters](https://github.com/plotters-rs/plotters/blob/master/plotters/examples/3d-plot.rs)

use std::time::Duration;

use eframe::egui::{self, CentralPanel, Visuals};
use egui::SidePanel;
use egui_plotter::{Chart, MouseConfig};
use plotters::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "3d Chart Example",
        native_options,
        Box::new(|cc| Ok(Box::new(Chart3d::new(cc)))),
    )
    .unwrap();
}

struct Chart3d {
    chart1: Chart<()>,
    chart2: Chart<()>,
}

impl Chart3d {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Enable light mode
        let context = &cc.egui_ctx;
        context.set_visuals(Visuals::light());

        // Create the first 3D chart
        let chart1 = Chart::new(())
            .mouse(MouseConfig::enabled())
            .pitch(0.7)
            .yaw(0.7)
            .builder_cb(Box::new(|area, transform, _d| {
                let x_axis = (-3.0..3.0).step(0.1);
                let z_axis = (-3.0..3.0).step(0.1);

                let mut chart = ChartBuilder::on(area)
                    .caption(format!("3D Plot Test"), (FontFamily::SansSerif, 20))
                    .build_cartesian_3d(x_axis, -3.0..3.0, z_axis)
                    .unwrap();

                chart.with_projection(|mut pb| {
                    pb.yaw = transform.yaw;
                    pb.pitch = transform.pitch;
                    pb.scale = transform.zoom;
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
                    .configure_series_labels()
                    .border_style(BLACK)
                    .draw()
                    .unwrap();
            }));

        let chart2 = Chart::new(())
            .mouse(MouseConfig::enabled())
            .pitch(0.7)
            .yaw(0.7)
            .builder_cb(Box::new(|area, transform, _d| {
                let x_axis = (-3.0..3.0).step(0.1);
                let z_axis = (-3.0..3.0).step(0.1);

                let mut chart = ChartBuilder::on(area)
                    .caption(format!("3D Plot Test"), (FontFamily::SansSerif, 20))
                    .build_cartesian_3d(x_axis, -3.0..3.0, z_axis)
                    .unwrap();

                chart.with_projection(|mut pb| {
                    pb.yaw = transform.yaw;
                    pb.pitch = transform.pitch;
                    pb.scale = transform.zoom;
                    pb.into_matrix()
                });

                chart
                    .configure_axes()
                    .light_grid_style(BLACK.mix(0.15))
                    .max_light_lines(3)
                    .draw()
                    .unwrap();

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
            }));

        Self { chart1, chart2 }
    }
}

impl eframe::App for Chart3d {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::right("chart 2").resizable(true).show(ctx, |ui| {
            ui.separator();
            self.chart2.draw(ui);
        });
        CentralPanel::default().show(ctx, |ui| {
            self.chart1.draw(ui);
        });

        // Limit framerate to 100fps
        std::thread::sleep(Duration::from_millis(10));
        ctx.request_repaint();
    }
}
