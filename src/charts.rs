//! Structs used to simplify the process of making interactive charts

use egui::Ui;
use plotters::prelude::{ChartBuilder, ChartContext, IntoDrawingArea};

use crate::EguiBackend;

const MOVE_SCALE: f32 = 0.01;
const SCROLL_SCALE: f32 = 0.001;

pub struct Chart3d {
    pitch: f64,
    yaw: f64,
    scale: f64,
    x: i32,
    y: i32,
    mouse: bool,
    builder_cb: Option<Box<dyn FnMut(ChartBuilder<EguiBackend>, Transform3d)>>,
}

pub struct Transform3d {
    pub pitch: f64,
    pub yaw: f64,
    pub scale: f64,
    pub x: i32,
    pub y: i32,
}

impl Chart3d {
    /// Create a new 3d chart with default settings.
    pub fn new() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            scale: 0.0,
            x: 0,
            y: 0,
            mouse: false,
            builder_cb: None,
        }
    }

    /// Enable or disable mouse controls.
    pub fn mouse(mut self, mouse: bool) -> Self {
        self.mouse = mouse;

        self
    }

    /// Set the builder callback
    pub fn builder_cb(
        mut self,
        builder_cb: Box<dyn FnMut(ChartBuilder<EguiBackend>, Transform3d)>,
    ) -> Self {
        self.builder_cb = Some(builder_cb);

        self
    }

    /// Draw the chart to a UI element
    pub fn draw(&mut self, ui: &Ui) {
        // First, get mouse data
        let (pitch_delta, yaw_delta, scale_delta) = ui.input(|input| {
            let pointer = &input.pointer;
            let delta = pointer.delta();

            let (pitch_delta, yaw_delta) = match pointer.primary_down() {
                true => (delta.y * MOVE_SCALE, -delta.x * MOVE_SCALE),
                false => (0.0, 0.0),
            };

            let scale_delta = input.scroll_delta.y * SCROLL_SCALE;

            (pitch_delta, yaw_delta, scale_delta)
        });

        self.pitch += pitch_delta as f64;
        self.yaw += yaw_delta as f64;
        self.scale += scale_delta as f64;

        let backend = EguiBackend::new(ui).into_drawing_area();

        let chart = ChartBuilder::on(&backend);

        match &mut self.builder_cb {
            Some(cb) => {
                let transform = Transform3d {
                    pitch: self.pitch,
                    yaw: self.yaw,
                    scale: self.scale,
                    x: self.x,
                    y: self.y,
                };

                cb(chart, transform);
            }

            None => {}
        }

        backend.present().unwrap();
    }
}
