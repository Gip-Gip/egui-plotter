//! Simple 3d plot example derived from
//! [eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
//! [plotters](https://github.com/plotters-rs/plotters/blob/master/plotters/examples/3d-plot.rs)

use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::EguiBackend;
use plotters::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;
        
        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let root = EguiBackend::new(ui).into_drawing_area();

            root.fill(&WHITE).unwrap();

            let x_axis = (-3.0..3.0).step(0.1);
            let z_axis = (-3.0..3.0).step(0.1);

            let mut chart = ChartBuilder::on(&root)
                .caption(format!("3D Plot Test"), (FontFamily::SansSerif, 20))
                .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone()).unwrap();

            chart.with_projection(|mut pb| {
                pb.yaw = 0.5;
                pb.scale = 0.9;
                pb.into_matrix()
            });

            chart
                .configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .max_light_lines(3)
                .draw().unwrap();

            chart
                .draw_series(
                    SurfaceSeries::xoz(
                        (-30..30).map(|f| f as f64 / 10.0),
                        (-30..30).map(|f| f as f64 / 10.0),
                        |x, z| (x * x + z * z).cos(),
                    )
                    .style(BLUE.mix(0.2).filled()),
                ).unwrap()
                .label("Surface")
                .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

            chart
                .draw_series(LineSeries::new(
                    (-100..100)
                        .map(|y| y as f64 / 40.0)
                        .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                    &BLACK,
                )).unwrap()
                .label("Line")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

            chart
                .configure_series_labels()
                .border_style(&BLACK)
                .draw().unwrap();

            // To avoid the IO failure being ignored silently, we manually call the present function
            root.present().unwrap();
        });
    }
}
