//! Animatable chart with data on the Y and time on the X axis

use egui::Ui;
use plotters::style::{RGBAColor, ShapeStyle};

use crate::charts::XyTimeData;

/// Animatabe chart with time on the X axis and data on the Y axis.
///
/// ## Usage
///
/// Creating the chart is very simple. You only need to provide 4 parameters,
/// 3 of which are just strings.
///
///  * `points`: A slice of tuples, arranged so that the first float is the time
///  and the second is the data.
///  * `unit`: String describing the data on the Y axis.
///  * `caption`: String to be shown as the caption of the chart.
///
/// This will create a basic line chart with nothing fancy, which you can easily
/// add to your egui project. You can also animate this chart with `.toggle_playback()`
/// and adjust various parameters with the many `.set_` functions included.
pub struct TimeData {
    chart: XyTimeData,
}

impl TimeData {
    /// Create a new TimeData chart. See [Usage](#usage).
    pub fn new(points: &[(f32, f32)], unit: &str, caption: &str) -> Self {
        let points: Vec<(f32, f32, f32)> = points
            .into_iter()
            .map(|(data, time)| (*data, *time, *time))
            .collect();

        let chart = XyTimeData::new(&points, "seconds", unit, caption);

        Self { chart }
    }

    /// Set the time to resume playback at. Time is in seconds.
    #[inline]
    pub fn set_time(&mut self, time: f32) {
        self.chart.set_time(time)
    }

    /// Set the time to resume playback at. Time is in seconds. Consumes self.
    #[inline]
    pub fn time(mut self, time: f32) -> Self {
        self.set_time(time);

        self
    }

    /// Set the playback speed. 1.0 is normal speed, 2.0 is double, & 0.5 is half.
    #[inline]
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.chart.set_playback_speed(speed)
    }

    /// Set the playback speed. 1.0 is normal speed, 2.0 is double, & 0.5 is half. Consumes self.
    #[inline]
    pub fn playback_speed(mut self, speed: f32) -> Self {
        self.set_playback_speed(speed);

        self
    }

    #[inline]
    /// Set the style of the plotted line.
    pub fn set_line_style(&mut self, line_style: ShapeStyle) {
        self.chart.set_line_style(line_style)
    }

    #[inline]
    /// Set the style of the plotted line. Consumes self.
    pub fn line_style(mut self, line_style: ShapeStyle) -> Self {
        self.set_line_style(line_style);

        self
    }

    #[inline]
    /// Set the style of the grid.
    pub fn set_grid_style(&mut self, grid_style: ShapeStyle) {
        self.chart.set_grid_style(grid_style)
    }

    #[inline]
    /// Set the style of the grid. Consumes self.
    pub fn grid_style(mut self, grid_style: ShapeStyle) -> Self {
        self.set_grid_style(grid_style);

        self
    }

    #[inline]
    /// Set the style of the subgrid.
    pub fn set_subgrid_style(&mut self, subgrid_style: ShapeStyle) {
        self.chart.set_subgrid_style(subgrid_style)
    }

    #[inline]
    /// Set the style of the subgrid. Consumes self.
    pub fn subgrid_style(mut self, subgrid_style: ShapeStyle) -> Self {
        self.set_subgrid_style(subgrid_style);

        self
    }

    #[inline]
    /// Set the style of the axes.
    pub fn set_axes_style(&mut self, axes_style: ShapeStyle) {
        self.chart.set_axes_style(axes_style)
    }

    #[inline]
    /// Set the style of the plotted line. Consumes self.
    pub fn axes_style(mut self, axes_style: ShapeStyle) -> Self {
        self.set_axes_style(axes_style);

        self
    }

    #[inline]
    /// Set the text color of the chart.
    pub fn set_text_color<T>(&mut self, color: T)
    where
        T: Into<RGBAColor>,
    {
        self.chart.set_text_color(color)
    }

    #[inline]
    /// Set the text color of the chart. Consumes self.
    pub fn text_color<T>(mut self, color: T) -> Self
    where
        T: Into<RGBAColor>,
    {
        self.set_text_color(color);

        self
    }

    #[inline]
    /// Set the background color of the chart.
    pub fn set_background_color<T>(&mut self, color: T)
    where
        T: Into<RGBAColor>,
    {
        self.chart.set_background_color(color);
    }

    #[inline]
    /// Set the background color of the chart. Consumes self.
    pub fn background_color<T>(mut self, color: T) -> Self
    where
        T: Into<RGBAColor>,
    {
        self.set_background_color(color);

        self
    }

    #[inline]
    /// Set the ratio between X and Y values, default being 1 x unit to 1 y unit.
    pub fn set_ratio(&mut self, ratio: f32) {
        self.chart.set_ratio(ratio);
    }

    #[inline]
    /// Set the ratio between X and Y values, default being 1 x unit to 1 y unit. Consumes self.
    pub fn ratio(mut self, ratio: f32) -> Self {
        self.set_ratio(ratio);

        self
    }

    /// Draw the chart to a Ui. Will also proceed to animate the chart if playback is currently
    /// enabled.
    pub fn draw(&mut self, ui: &Ui) {
        self.chart.draw(ui)
    }

    /// Start/enable playback of the chart.
    #[inline]
    pub fn start_playback(&mut self) {
        self.chart.start_playback()
    }

    /// Stop/disable playback of the chart.
    #[inline]
    pub fn stop_playback(&mut self) {
        self.chart.stop_playback()
    }

    /// Toggle playback of the chart.
    pub fn toggle_playback(&mut self) {
        self.chart.toggle_playback()
    }

    /// Return true if playback is currently enabled & underway.
    #[inline]
    pub fn is_playing(&self) -> bool {
        self.chart.is_playing()
    }

    /// Return the time the chart starts at when playback is enabled.
    #[inline]
    pub fn start_time(&self) -> f32 {
        self.chart.start_time()
    }

    /// Return the current time to be animated when playback is enabled.
    #[inline]
    pub fn current_time(&mut self) -> f32 {
        self.chart.current_time()
    }

    /// Return the time the chart finished animating at when playback is enabled.
    #[inline]
    pub fn end_time(&self) -> f32 {
        self.chart.end_time()
    }

    /// Return the speed the chart is animated at.
    #[inline]
    pub fn get_playback_speed(&self) -> f32 {
        self.chart.get_playback_speed()
    }
}
