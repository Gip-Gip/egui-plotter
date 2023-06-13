//! Structs used to simplify the process of making interactive charts

use egui::Ui;
use plotters::prelude::{ChartBuilder, ChartContext, IntoDrawingArea};

use crate::EguiBackend;

const MOVE_SCALE: f32 = 0.01;
const SCROLL_SCALE: f32 = 0.001;

pub struct Chart3d {
    transform: Transform3d,
    mouse: bool,
    builder_cb: Option<Box<dyn FnMut(ChartBuilder<EguiBackend>, &Transform3d)>>,
}

#[derive(Debug, Default, Copy, Clone)]
/// Struct used to apply transformations to 3d charts
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
            transform: Transform3d::default(),
            mouse: false,
            builder_cb: None,
        }
    }

    #[inline]
    /// Enable or disable mouse controls.
    pub fn set_mouse(&mut self, mouse: bool) {
        self.mouse = mouse
    }

    #[inline]
    /// Enable or disable mouse controls. Consumes self.
    pub fn mouse(mut self, mouse: bool) -> Self {
        self.set_mouse(mouse);

        self
    }

    #[inline]
    /// Set the builder callback
    pub fn set_builder_cb(
        &mut self,
        builder_cb: Box<dyn FnMut(ChartBuilder<EguiBackend>, &Transform3d)>,
    ) {
        self.builder_cb = Some(builder_cb)
    }

    #[inline]
    /// Set the builder callback. Consumes self.
    pub fn builder_cb(
        mut self,
        builder_cb: Box<dyn FnMut(ChartBuilder<EguiBackend>, &Transform3d)>,
    ) -> Self {
        self.set_builder_cb(builder_cb);

        self
    }

    #[inline]
    /// Set the pitch of the chart.
    pub fn set_pitch(&mut self, pitch: f64) {
        self.transform.pitch = pitch
    }

    #[inline]
    /// Set the pitch of the chart. Consumes self.
    pub fn pitch(mut self, pitch: f64) -> Self {
        self.set_pitch(pitch);

        self
    }

    #[inline]
    /// Set the yaw of the chart.
    pub fn set_yaw(&mut self, yaw: f64) {
        self.transform.yaw = yaw
    }

    #[inline]
    /// Set the yaw of the chart. Consumes self.
    pub fn yaw(mut self, yaw: f64) -> Self {
        self.set_yaw(yaw);

        self
    }

    #[inline]
    /// Set the scale of the chart.
    pub fn set_scale(&mut self, scale: f64) {
        self.transform.scale = scale
    }

    #[inline]
    /// Set the scale of the chart. Consumes self.
    pub fn scale(mut self, scale: f64) -> Self {
        self.set_scale(scale);

        self
    }

    /// Draw the chart to a UI element
    pub fn draw(&mut self, ui: &Ui) {
        let transform = &mut self.transform;

        // First, get mouse data if mouse control is enabled
        if self.mouse {
            ui.input(|input| {
                let pointer = &input.pointer;
                let delta = pointer.delta();

                // Adjust the pitch/yaw if the primary button is pressed
                if pointer.middle_down() {
                    let pitch_delta = delta.y * MOVE_SCALE;
                    let yaw_delta = delta.x * MOVE_SCALE;

                    transform.pitch += pitch_delta as f64;
                    transform.yaw += -yaw_delta as f64;
                }

                // Adjust the x/y if the middle button is down
                if pointer.primary_down() {
                    let x_delta = delta.x;
                    let y_delta = delta.y;

                    transform.x += x_delta as i32;
                    transform.y += y_delta as i32;
                }

                let scale_delta = input.scroll_delta.y * SCROLL_SCALE;

                transform.scale += scale_delta as f64;
            });
        }

        let backend = EguiBackend::new(ui)
            .offset((transform.x, transform.y))
            .into_drawing_area();

        let chart = ChartBuilder::on(&backend);

        match &mut self.builder_cb {
            Some(cb) => {
                cb(chart, transform);
            }

            None => {}
        }

        backend.present().unwrap();
    }
}
