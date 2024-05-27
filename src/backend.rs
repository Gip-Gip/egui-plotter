//! Plotter backend for egui

use std::error::Error as ErrorTrait;
use std::f32::consts::FRAC_PI_2;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, MulAssign, Sub, SubAssign};

use egui::{
    Mesh,
    epaint::{PathShape, TextShape, Vertex},
    Align, Align2, Color32, FontFamily as EguiFontFamily, FontId, Pos2, Rect, Stroke, Ui,
};
use plotters_backend::{
    text_anchor::{HPos, Pos, VPos},
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
    FontFamily as PlottersFontFamily,
};

#[derive(Debug, Clone, Copy)]
/// Error to be returned by the backend. Since egui doesn't return any errors
/// on any painter operations, this is a stub type.
pub struct EguiBackendError;

impl Display for EguiBackendError {
    #[inline]
    fn fmt(&self, _f: &mut Formatter<'_>) -> FmtResult {
        Ok(())
    }
}

impl ErrorTrait for EguiBackendError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
/// Struct used to convert between egui's Pos2 type and plotter's coordinate tuple.
/// Also provides implementations for adding coordinates together.
struct EguiBackendCoord {
    x: f32,
    y: f32,
}

impl From<(i32, i32)> for EguiBackendCoord {
    #[inline]
    fn from(value: (i32, i32)) -> Self {
        let (x, y) = value;

        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}

impl From<EguiBackendCoord> for Pos2 {
    #[inline]
    fn from(val: EguiBackendCoord) -> Self {
        Pos2 { x: val.x, y: val.y }
    }
}

impl From<EguiBackendCoord> for (u32, u32) {
    #[inline]
    fn from(val: EguiBackendCoord) -> Self {
        (val.x as u32, val.y as u32)
    }
}

impl From<Pos2> for EguiBackendCoord {
    #[inline]
    fn from(value: Pos2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Add<EguiBackendCoord> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: EguiBackendCoord) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<EguiBackendCoord> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn sub(self, rhs: EguiBackendCoord) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Pos2> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: Pos2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<EguiBackendCoord> for EguiBackendCoord {
    fn add_assign(&mut self, rhs: EguiBackendCoord) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<EguiBackendCoord> for EguiBackendCoord {
    fn sub_assign(&mut self, rhs: EguiBackendCoord) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<f32> for EguiBackendCoord {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[derive(Debug, Clone, Copy)]
/// Struct used to convert between Egui and Plotter's color types
struct EguiBackendColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<BackendColor> for EguiBackendColor {
    #[inline]
    fn from(value: BackendColor) -> Self {
        let (r, g, b) = value.rgb;

        let a = (value.alpha * 255.0) as u8;

        Self { r, g, b, a }
    }
}

impl From<EguiBackendColor> for Color32 {
    #[inline]
    fn from(val: EguiBackendColor) -> Self {
        Color32::from_rgba_unmultiplied(val.r, val.g, val.b, val.a)
    }
}

/// Plotter backend for egui; simply provide a reference to the ui element to
/// use.
pub struct EguiBackend<'a> {
    ui: &'a Ui,
    x: i32,
    y: i32,
    scale: f32,
}

impl<'a> EguiBackend<'a> {
    #[inline]
    /// Create a backend given a reference to a Ui.
    pub fn new(ui: &'a Ui) -> Self {
        Self {
            ui,
            x: 0,
            y: 0,
            scale: 1.0,
        }
    }

    #[inline]
    /// Transform point
    fn point_transform(&self, mut point: EguiBackendCoord, bounds: Rect) -> EguiBackendCoord {
        let center = EguiBackendCoord::from(bounds.center()) - EguiBackendCoord::from(bounds.min);
        point -= center;
        point *= self.scale;
        point += center;

        point += EguiBackendCoord::from((self.x, self.y));
        point += EguiBackendCoord::from(bounds.min);

        point
    }
    #[inline]
    /// Set the offset(x + y) of the backend.
    pub fn set_offset(&mut self, offset: (i32, i32)) {
        (self.x, self.y) = offset
    }

    #[inline]
    /// Set the offset(x + y) of the backend. Consumes self.
    pub fn offset(mut self, offset: (i32, i32)) -> Self {
        self.set_offset(offset);

        self
    }

    #[inline]
    /// Set the scale of the backend.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale
    }

    #[inline]
    /// Set the scale of the backend. Consume self.
    pub fn scale(mut self, scale: f32) -> Self {
        self.set_scale(scale);

        self
    }
}

impl<'a> DrawingBackend for EguiBackend<'a> {
    type ErrorType = std::io::Error;

    fn get_size(&self) -> (u32, u32) {
        let bounds = self.ui.max_rect();
        (bounds.width() as u32, bounds.height() as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: (i32, i32),
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter().with_clip_rect(bounds);

        let p0 = self.point_transform(EguiBackendCoord::from(point), bounds);

        let p1 = p0 + 1.0;

        let color: Color32 = EguiBackendColor::from(color).into();

        let stroke = Stroke::new(1.0, color);

        painter.line_segment([p0.into(), p1.into()], stroke);

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: (i32, i32),
        to: (i32, i32),
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter().with_clip_rect(bounds);

        let p0 = self.point_transform(EguiBackendCoord::from(from), bounds);
        let p1 = self.point_transform(EguiBackendCoord::from(to), bounds);

        let color: Color32 = EguiBackendColor::from(style.color()).into();

        let stroke = Stroke::new(style.stroke_width() as f32, color);

        painter.line_segment([p0.into(), p1.into()], stroke);

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: (i32, i32),
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter().with_clip_rect(bounds);

        let pos = self.point_transform(EguiBackendCoord::from(pos), bounds);

        let font_size = style.size() as f32;
        let font_family = match style.family() {
            PlottersFontFamily::Serif | PlottersFontFamily::SansSerif => {
                EguiFontFamily::Proportional
            }
            PlottersFontFamily::Monospace => EguiFontFamily::Monospace,
            PlottersFontFamily::Name(string) => EguiFontFamily::Name(string.into()),
        };

        let font = FontId {
            size: font_size,
            family: font_family,
        };

        let color: Color32 = EguiBackendColor::from(style.color()).into();

        let rotations = style.transform() as usize;
        let angle = rotations as f32 * FRAC_PI_2;

        let Pos { h_pos, v_pos } = style.anchor();

        // !TODO! Find a slightly more eligant rotation function.
        let mut anchor = Align2([
            match h_pos {
                HPos::Left => Align::LEFT,
                HPos::Right => Align::RIGHT,
                HPos::Center => Align::Center,
            },
            match v_pos {
                VPos::Top => Align::TOP,
                VPos::Center => Align::Center,
                VPos::Bottom => Align::BOTTOM,
            },
        ]);
        fn rotate(anchor: &mut Align2) {
            *anchor = match anchor {
                &mut Align2::LEFT_TOP => Align2::RIGHT_TOP,
                &mut Align2::RIGHT_TOP => Align2::RIGHT_BOTTOM,
                &mut Align2::RIGHT_BOTTOM => Align2::LEFT_BOTTOM,
                &mut Align2::LEFT_BOTTOM => Align2::LEFT_TOP,
                &mut Align2::LEFT_CENTER => Align2::CENTER_TOP,
                &mut Align2::CENTER_TOP => Align2::RIGHT_CENTER,
                &mut Align2::RIGHT_CENTER => Align2::CENTER_BOTTOM,
                &mut Align2::CENTER_BOTTOM => Align2::LEFT_CENTER,
                &mut Align2::CENTER_CENTER => Align2::CENTER_CENTER,
            }
        }
        for _ in 0..rotations {
            rotate(&mut anchor)
        }
        let galley = painter.layout_no_wrap(text.to_string(), font, color);
        let rect = anchor.anchor_rect(Rect::from_min_size(pos.into(), galley.size()));
        if !galley.is_empty() {
            painter.add(TextShape {
                angle,
                ..TextShape::new(rect.min, galley, Color32::PLACEHOLDER)
            });
        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter().with_clip_rect(bounds);

        let points: Vec<Pos2> = path
            .into_iter()
            .map(|point| {
                let point = self.point_transform(EguiBackendCoord::from(point), bounds);

                point.into()
            })
            .collect();

        let color: Color32 = EguiBackendColor::from(style.color()).into();

        let stroke = Stroke::new(style.stroke_width() as f32, color);

        let shape = PathShape::line(points, stroke);

        painter.add(shape);
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter().with_clip_rect(bounds);

        let vertices: Vec<Vertex> = vert
            .into_iter()
            .map(|coord| Vertex {
                pos: self
                    .point_transform(EguiBackendCoord::from(coord), bounds)
                    .into(),
                uv: Default::default(),
                color: EguiBackendColor::from(style.color()).into(),
            })
            .collect();

        let flat_vertices: Vec<f32> = vertices
            .iter()
            .flat_map(|v| vec![v.pos.x, v.pos.y])
            .collect();

        let mut mesh = Mesh::default();
        mesh.vertices.extend(vertices);

        earcutr::earcut(&flat_vertices, &Vec::new(), 2)
            .map_err(|e| {
                DrawingErrorKind::DrawingError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("earcutr error: {}", e),
                ))
            })?
            .chunks_exact(3)
            .for_each(|triangle| {
                mesh.add_triangle(triangle[0] as u32, triangle[1] as u32, triangle[2] as u32);
            });

        painter.add(mesh);

        Ok(())
    }
}
