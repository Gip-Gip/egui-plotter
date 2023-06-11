use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Add;

use egui::{FontFamily as EguiFontFamily, Ui};
use emath::Align2;
use epaint::{Color32, FontId, PathShape, Pos2, Stroke};
use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
    FontFamily as PlottersFontFamily,
};

#[derive(Debug, Clone, Copy)]
/// Error to be returned by the backend. Since egui doesn't return any errors
/// on any painter operations, this is a stub type.
pub enum EguiBackendError {
    None,
}

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

impl Into<Pos2> for EguiBackendCoord {
    #[inline]
    fn into(self) -> Pos2 {
        Pos2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<(u32, u32)> for EguiBackendCoord {
    #[inline]
    fn into(self) -> (u32, u32) {
        (self.x as u32, self.y as u32)
    }
}

impl Add<EguiBackendCoord> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: EguiBackendCoord) -> Self::Output {
        let sum = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };

        sum
    }
}

impl Add<Pos2> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: Pos2) -> Self::Output {
        let sum = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };

        sum
    }
}

impl Add<f32> for EguiBackendCoord {
    type Output = EguiBackendCoord;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let sum = Self {
            x: self.x + rhs,
            y: self.y + rhs,
        };

        sum
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

impl Into<Color32> for EguiBackendColor {
    #[inline]
    fn into(self) -> Color32 {
        Color32::from_rgba_unmultiplied(self.r, self.g, self.b, self.a)
    }
}

/// Plotter backend for egui; simply provide a reference to the ui element to
/// use.
pub struct EguiBackend<'a> {
    ui: &'a Ui,
}

impl<'a> EguiBackend<'a> {
    #[inline]
    pub fn new(ui: &'a Ui) -> Self {
        Self { ui }
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
        let painter = self.ui.painter();

        let p0 = EguiBackendCoord::from(point) + bounds.min;

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
        let painter = self.ui.painter();

        let p0 = EguiBackendCoord::from(from) + bounds.min;
        let p1 = EguiBackendCoord::from(to) + bounds.min;

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
        let painter = self.ui.painter();

        let pos = EguiBackendCoord::from(pos) + bounds.min;

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

        painter.text(pos.into(), Align2::LEFT_TOP, text, font, color);

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let bounds = self.ui.max_rect();
        let painter = self.ui.painter();

        let points: Vec<Pos2> = path
            .into_iter()
            .map(|point| {
                let point = EguiBackendCoord::from(point) + bounds.min;

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
        let painter = self.ui.painter();

        let points: Vec<Pos2> = vert
            .into_iter()
            .map(|point| {
                let point = EguiBackendCoord::from(point) + bounds.min;

                point.into()
            })
            .collect();

        let color: Color32 = EguiBackendColor::from(style.color()).into();

        let stroke = Stroke::NONE;

        let shape = PathShape::convex_polygon(points, color, stroke);

        painter.add(shape);

        Ok(())
    }
}
