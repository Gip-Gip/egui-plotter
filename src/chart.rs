//! Structs used to simplify the process of making interactive charts

use egui::{PointerState, Ui};
use plotters::{
    coord::Shift,
    prelude::{DrawingArea, IntoDrawingArea},
};

use crate::EguiBackend;

/// Default pitch and yaw scale for mouse rotations.
pub const DEFAULT_MOVE_SCALE: f32 = 0.01;
/// Default zoom scale for scroll wheel zooming.
pub const DEFAULT_SCROLL_SCALE: f32 = 0.001;

#[derive(Debug, Copy, Clone)]
/// Transformations to be applied to your chart. Is modified by user input(if the mouse is enabled) and
/// used by Chart::draw() and your builder callback.
///
/// Chart::draw() applies the scale and the x/y offset to your plot, so unless
/// you want to create some effects on your own you don't need to worry about them.
///
/// If you are creating a 3d plot however you will have to manually apply the pitch and
/// yaw to your chart with the following code:
///
/// ```ignore
/// chart.with_projection(|mut pb| {
///     pb.yaw = transform.yaw;
///     pb.pitch = transform.pitch;
///     pb.scale = 0.7; // Set scale to 0.7 to avoid artifacts caused by plotter's renderer
///     pb.into_matrix()
/// });
///```
pub struct Transform {
    /// Pitch of your graph in 3d
    pub pitch: f64,
    /// Yaw of your graph in 3d
    pub yaw: f64,
    /// Scale of your graph. Applied in Chart::draw()
    pub scale: f64,
    /// X offset of your graph. Applied in Chart::draw()
    pub x: i32,
    /// Y offset of your graph. Applied in Chart::draw()
    pub y: i32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            scale: 1.0,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Mouse buttons that can be bound to chart actions
pub enum MouseButton {
    Primary,
    Middle,
    Secondary,
}

impl MouseButton {
    /// See if the mouse button is down given a PointerState
    pub fn is_down(&self, pointer: &PointerState) -> bool {
        match self {
            Self::Primary => pointer.primary_down(),
            Self::Middle => pointer.middle_down(),
            Self::Secondary => pointer.secondary_down(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Used to configure how the mouse interacts with the chart.
///
/// ## Usage
/// MouseConfig allows you to change the ways the user interacts with your chart in the following
/// ways:
///  * `drag`, `rotate`, & `zoom` - Enables dragging, rotating, and zooming in on your plots with
///  mouse controls.
///  * `pitch_scale` & `yaw_scale` - Modifies how quickly the pitch and yaw are rotated when rotating with the
///  mouse.
///  * `zoom_scale` - Modifies how quickly you zoom in/out.
///  * `drag_bind` - Mouse button bound to dragging your plot.
///  * `rotate_bind` - Mouse button bound to rotating your plot.
pub struct MouseConfig {
    drag: bool,
    rotate: bool,
    zoom: bool,
    yaw_scale: f32,
    pitch_scale: f32,
    zoom_scale: f32,
    drag_bind: MouseButton,
    rotate_bind: MouseButton,
}

impl Default for MouseConfig {
    fn default() -> Self {
        Self {
            drag: false,
            rotate: false,
            zoom: false,
            yaw_scale: DEFAULT_MOVE_SCALE,
            pitch_scale: DEFAULT_MOVE_SCALE,
            zoom_scale: DEFAULT_SCROLL_SCALE,
            drag_bind: MouseButton::Middle,
            rotate_bind: MouseButton::Primary,
        }
    }
}

impl MouseConfig {
    #[inline]
    /// Create a new MouseConfig with dragging, rotationg, and zooming enabled.
    pub fn enabled() -> Self {
        Self {
            drag: true,
            rotate: true,
            zoom: true,
            yaw_scale: DEFAULT_MOVE_SCALE,
            pitch_scale: DEFAULT_MOVE_SCALE,
            zoom_scale: DEFAULT_SCROLL_SCALE,
            drag_bind: MouseButton::Middle,
            rotate_bind: MouseButton::Primary,
        }
    }

    #[inline]
    /// Enables dragging, rotating, and zooming in on your plots.
    fn set_enable_all(&mut self) {
        self.set_drag(true);
        self.set_zoom(true);
        self.set_rotate(true);
    }

    #[inline]
    /// Enables dragging, rotating, and zooming in on your plots. Consumes self.
    pub fn enable_all(mut self) -> Self {
        self.set_enable_all();

        self
    }

    #[inline]
    /// Enable/disable dragging of the chart.
    pub fn set_drag(&mut self, drag: bool) {
        self.drag = drag
    }

    #[inline]
    /// Enable/disable dragging of the chart. Consumes self.
    pub fn drag(mut self, drag: bool) -> Self {
        self.set_drag(drag);

        self
    }

    #[inline]
    /// Enable/disable rotation of the chart.
    pub fn set_rotate(&mut self, rotate: bool) {
        self.rotate = rotate
    }

    #[inline]
    /// Enable/disable rotation of the chart. Consumes self.
    pub fn rotate(mut self, rotate: bool) -> Self {
        self.set_rotate(rotate);

        self
    }

    #[inline]
    /// Enable/disable zoom of the chart.
    pub fn set_zoom(&mut self, zoom: bool) {
        self.zoom = zoom;
    }

    #[inline]
    /// Enable/disable zoom of the chart. Consumes self.
    pub fn zoom(mut self, zoom: bool) -> Self {
        self.set_zoom(zoom);

        self
    }

    #[inline]
    /// Change the pitch scale.
    pub fn set_pitch_scale(&mut self, scale: f32) {
        self.pitch_scale = scale
    }

    #[inline]
    /// Change the pitch scale. Consumes self.
    pub fn pitch_scale(mut self, scale: f32) -> Self {
        self.set_pitch_scale(scale);

        self
    }
}

/// Allows users to drag, rotate, and zoom in/out on your plots.
///
/// ## Usage
/// Charts are designed to be easy to implement and use, while simultaniously
/// being powerful enough for your application. You can manipulate the
/// following properties of a chart to get the effects you want:
///  * `builder_cb` - Callback used to populate the chart. Is provided a DrawingArea and the
///  chart's `data`.
///  * `mouse` - Mouse configuration. Configure how you wish the mouse to affect/manipulate the
///  chart.
///  * `data` - A Box of data of any type to be stored with the chart. Provided so that you can modify data
///  without having to specify a new callback during runtime. For example, `examples/parachart.rs`
///  uses it to store the range so it can be changed during runtime.
///
///  ## Examples
///  See `examples/3dchart.rs` and `examples/parachart.rs` for examples of usage.
pub struct Chart<Data> {
    transform: Transform,
    mouse: MouseConfig,
    builder_cb: Option<Box<dyn FnMut(&mut DrawingArea<EguiBackend, Shift>, &Transform, &Data)>>,
    data: Data,
}

impl<Data> Chart<Data> {
    /// Create a new chart with default settings (if not using data supply ())
    pub fn new(data: Data) -> Self {
        Self {
            transform: Transform::default(),
            mouse: MouseConfig::default(),
            builder_cb: None,
            data,
        }
    }

    #[inline]
    /// Enable or disable mouse controls.
    pub fn set_mouse(&mut self, mouse: MouseConfig) {
        self.mouse = mouse
    }

    #[inline]
    /// Enable or disable mouse controls. Consumes self.
    pub fn mouse(mut self, mouse: MouseConfig) -> Self {
        self.set_mouse(mouse);

        self
    }

    #[inline]
    /// Set the builder callback.
    pub fn set_builder_cb(
        &mut self,
        builder_cb: Box<dyn FnMut(&mut DrawingArea<EguiBackend, Shift>, &Transform, &Data)>,
    ) {
        self.builder_cb = Some(builder_cb)
    }

    #[inline]
    /// Set the builder callback. Consumes self.
    pub fn builder_cb(
        mut self,
        builder_cb: Box<dyn FnMut(&mut DrawingArea<EguiBackend, Shift>, &Transform, &Data)>,
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

    #[inline]
    /// Get the data of the chart as a reference.
    pub fn get_data(&self) -> &Data {
        &self.data
    }

    #[inline]
    /// Get the data of the chart as a mutable reference.
    pub fn get_data_mut(&mut self) -> &mut Data {
        &mut self.data
    }

    /// Call the callback and draw the chart to a UI element.
    pub fn draw(&mut self, ui: &Ui) {
        let transform = &mut self.transform;

        // First, get mouse data
        ui.input(|input| {
            let pointer = &input.pointer;
            let delta = pointer.delta();

            // Adjust the pitch/yaw if the primary button is pressed and rotation is enabled
            if self.mouse.rotate && self.mouse.rotate_bind.is_down(pointer) {
                let pitch_delta = delta.y * self.mouse.pitch_scale;
                let yaw_delta = delta.x * self.mouse.yaw_scale;

                transform.pitch += pitch_delta as f64;
                transform.yaw += -yaw_delta as f64;
            }

            // Adjust the x/y if the middle button is down and dragging is enabled
            if self.mouse.drag && self.mouse.drag_bind.is_down(pointer) {
                let x_delta = delta.x;
                let y_delta = delta.y;

                transform.x += x_delta as i32;
                transform.y += y_delta as i32;
            }

            // Adjust zoom if zoom is enabled
            if self.mouse.zoom {
                let scale_delta = input.smooth_scroll_delta.y * self.mouse.zoom_scale;

                // !TODO! make scaling exponential
                transform.scale = (transform.scale + scale_delta as f64).abs();
            }
        });

        let mut area = EguiBackend::new(ui)
            .offset((transform.x, transform.y))
            .scale(transform.scale as f32)
            .into_drawing_area();

        if let Some(cb) = &mut self.builder_cb {
            cb(&mut area, transform, &self.data);
        }

        area.present().unwrap();
    }
}
