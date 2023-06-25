//! Animatable line chart. Can have X and Y points.

use std::{
    cmp::Ordering,
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};

use egui::Ui;
use plotters::{
    prelude::ChartBuilder,
    series::LineSeries,
    style::{
        full_palette::{GREY, GREY_700, RED_900},
        Color, FontDesc, RGBAColor, ShapeStyle, TextStyle, BLACK, WHITE,
    },
};
use plotters_backend::{FontFamily, FontStyle};

use crate::{mult_range, Chart, MouseConfig};

const MIN_DELTA: f32 = 0.000_010;
const DEFAULT_RATIO: f32 = 1.0;
const X_MARGIN: i32 = 25;
const Y_MARGIN: i32 = 25;
const LABEL_AREA: i32 = 25;
const CAPTION_SIZE: i32 = 10;

#[derive(Clone)]
struct XyTimeConfig {
    /// Points to be plotted. A slice of X, Y f32 pairs.
    points: Arc<[(f32, f32)]>,
    /// Ranges at different time points.
    range: (Range<f32>, Range<f32>),
    /// Style of the plotted line.
    line_style: ShapeStyle,
    /// Style of the grid lines.
    grid_style: ShapeStyle,
    /// Style of the small grid lines.
    subgrid_style: ShapeStyle,
    /// Style of the axes.
    axes_style: ShapeStyle,
    /// Style of the text
    text_color: RGBAColor,
    /// Background color of the chart.
    background_color: RGBAColor,
    /// Unit of the X axis.
    x_unit: Arc<str>,
    /// Unit of the Y axis.
    y_unit: Arc<str>,
    /// Ratio between the X and Y axis units.
    ratio: f32,
    /// Caption of the chart.
    caption: Arc<str>,
}

/// Animatable 2d line chart.
///
/// ## Usage
///
/// Creating the chart is very simple. You only need to provide 4 parameters,
/// 3 of which are just strings.
///
///  * `points`: A slice of tuples, arranged so that the first float is the x position, the second
///  the y position, and the third is the time the next point is to be shown at(or in the case of
///  the last point, the time the animation ends).
///  * `x_unit`: String describing the data on the X axis.
///  * `y_unit`: String describing the data on the Y axis.
///  * `caption`: String to be shown as the caption of the chart.
///
/// This will create a basic line chart with nothing fancy, which you can easily
/// add to your egui project. You can also animate this chart with `.toggle_playback()`
/// and adjust various parameters with the many `.set_` functions included.
pub struct XyTimeData {
    playback_start: Option<Instant>,
    pause_start: Option<Instant>,
    playback_speed: f32,
    points: Arc<[(f32, f32)]>,
    ranges: Arc<[(Range<f32>, Range<f32>)]>,
    times: Arc<[f32]>,
    chart: Chart<XyTimeConfig>,
}

impl XyTimeData {
    /// Create a new XyTimeData chart. See [Usage](#usage).
    pub fn new(points: &[(f32, f32, f32)], x_unit: &str, y_unit: &str, caption: &str) -> Self {
        let mut points = points.to_vec();

        // Sort by the time of the point
        points.sort_by(|a, b| {
            let (_, _, a) = a;
            let (_, _, b) = b;

            a.partial_cmp(b).unwrap_or(Ordering::Equal)
        });

        let times: Vec<f32> = points
            .iter()
            .map(|point| {
                let (_, _, time) = point;

                *time
            })
            .collect();

        let points: Vec<(f32, f32)> = points
            .iter()
            .map(|point| {
                let (x, y, _) = point;

                (*x, *y)
            })
            .collect();

        // Ranges include the X range, Y range, and time in seconds
        let mut ranges = Vec::<(Range<f32>, Range<f32>)>::with_capacity(points.len());

        let mut min_x: f32 = f32::MAX;
        let mut min_y: f32 = f32::MAX;
        let mut max_x: f32 = f32::MIN;
        let mut max_y: f32 = f32::MIN;

        for point in &points {
            let (x, y) = *point;

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);

            let range_x = min_x..max_x;
            let range_y = min_y..max_y;

            ranges.push((range_x, range_y));
        }

        let y_unit: String = y_unit.split("").map(|c| format!("{}\n", c)).collect();

        // Turn all the vecs and strings into arcs since they are more or less read-only at
        // this point

        let points: Arc<[(f32, f32)]> = points.into();
        let ranges: Arc<[(Range<f32>, Range<f32>)]> = ranges.into();
        let times: Arc<[f32]> = times.into();

        let x_unit: Arc<str> = x_unit.into();
        let y_unit: Arc<str> = y_unit.into();
        let caption: Arc<str> = caption.into();

        let grid_style = ShapeStyle {
            color: GREY.to_rgba(),
            filled: false,
            stroke_width: 2,
        };

        let subgrid_style = ShapeStyle {
            color: GREY_700.to_rgba(),
            filled: false,
            stroke_width: 1,
        };

        let axes_style = ShapeStyle {
            color: BLACK.to_rgba(),
            filled: false,
            stroke_width: 2,
        };

        let line_style = ShapeStyle {
            color: RED_900.to_rgba(),
            filled: false,
            stroke_width: 2,
        };

        let background_color = WHITE.to_rgba();
        let text_color = BLACK.to_rgba();

        let config = XyTimeConfig {
            points: points.clone(),
            range: ranges.last().unwrap().clone(),
            line_style,
            grid_style,
            subgrid_style,
            axes_style,
            text_color,
            background_color,
            x_unit,
            y_unit,
            ratio: DEFAULT_RATIO,
            caption,
        };

        let chart = Chart::new(config)
            .mouse(MouseConfig::enabled())
            .builder_cb(Box::new(|area, _t, data| {
                let area_ratio = {
                    let (x_range, y_range) = area.get_pixel_range();

                    let x_delta =
                        ((x_range.end - x_range.start).abs() - (X_MARGIN * 2) - LABEL_AREA) as f32;
                    let y_delta = ((y_range.end - y_range.start).abs()
                        - (Y_MARGIN * 2)
                        - LABEL_AREA
                        - CAPTION_SIZE) as f32;

                    x_delta / y_delta
                };

                // Return if the ratio is invalid(meaning the chart can't be drawn)
                if !area_ratio.is_finite() {
                    return;
                }

                let (x_range, y_range) = data.range.clone();

                // The data ratio is inverse, as if our X range is smaller we
                // want to make sure the X axis is expanded to compensate
                let data_ratio = {
                    let x_delta = (x_range.end - x_range.start).abs();
                    let y_delta = (y_range.end - y_range.start).abs();

                    y_delta / x_delta
                };

                let display_ratio = data.ratio * data_ratio * area_ratio;

                let (x_range, y_range) =
                    match display_ratio.partial_cmp(&1.0).unwrap_or(Ordering::Equal) {
                        Ordering::Equal => (x_range, y_range),
                        Ordering::Greater => (mult_range(x_range, display_ratio), y_range),
                        Ordering::Less => (x_range, mult_range(y_range, 1.0 / display_ratio)),
                    };

                let font_style = FontStyle::Normal;
                let font_family = FontFamily::Monospace;
                let font_size = CAPTION_SIZE;

                let font_desc = FontDesc::new(font_family, font_size as f64, font_style);

                let text_style = TextStyle::from(font_desc).color(&data.text_color);

                let mut chart = ChartBuilder::on(area)
                    .caption(data.caption.clone(), text_style.clone())
                    .x_label_area_size(LABEL_AREA)
                    .y_label_area_size(LABEL_AREA)
                    .margin_left(X_MARGIN)
                    .margin_right(X_MARGIN)
                    .margin_top(Y_MARGIN)
                    .margin_bottom(Y_MARGIN)
                    .build_cartesian_2d(x_range, y_range)
                    .unwrap();

                chart
                    .configure_mesh()
                    .label_style(text_style.clone())
                    .bold_line_style(data.grid_style)
                    .light_line_style(data.subgrid_style)
                    .axis_style(data.axes_style)
                    .x_desc(&data.x_unit.to_string())
                    .set_all_tick_mark_size(4)
                    .y_desc(&data.y_unit.to_string())
                    .draw()
                    .unwrap();

                chart
                    .draw_series(LineSeries::new(data.points.to_vec(), data.line_style))
                    .unwrap();
            }));

        Self {
            playback_start: None,
            pause_start: None,
            playback_speed: 1.0,
            points,
            ranges,
            times,
            chart,
        }
    }

    /// Set the time to resume playback at. Time is in seconds.
    pub fn set_time(&mut self, time: f32) {
        let start_time = Some(Instant::now() - Duration::from_secs_f32(time));
        match self.playback_start {
            Some(_) => {
                if let Some(_) = self.pause_start {
                    self.pause_start = Some(Instant::now());
                }

                self.playback_start = start_time;
            }
            None => {
                self.playback_start = start_time;
                self.pause_start = Some(Instant::now());
            }
        }
    }

    #[inline]
    /// Set the time to resume playback at. Time is in seconds. Consumes self.
    pub fn time(mut self, time: f32) -> Self {
        self.set_time(time);

        self
    }

    #[inline]
    /// Set the playback speed. 1.0 is normal speed, 2.0 is double, & 0.5 is half.
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.playback_speed = speed;
    }

    #[inline]
    /// Set the playback speed. 1.0 is normal speed, 2.0 is double, & 0.5 is half. Consumes self.
    pub fn playback_speed(mut self, speed: f32) -> Self {
        self.set_playback_speed(speed);

        self
    }

    /// Set the style of the plotted line.
    pub fn set_line_style(&mut self, line_style: ShapeStyle) {
        self.chart.set_data(|config| {
            config.line_style = line_style;
        })
    }

    #[inline]
    /// Set the style of the plotted line. Consumes self.
    pub fn line_style(mut self, line_style: ShapeStyle) -> Self {
        self.set_line_style(line_style);

        self
    }

    /// Set the style of the grid.
    pub fn set_grid_style(&mut self, grid_style: ShapeStyle) {
        self.chart.set_data(|config| config.grid_style = grid_style)
    }

    #[inline]
    /// Set the style of the grid. Consumes self.
    pub fn grid_style(mut self, grid_style: ShapeStyle) -> Self {
        self.set_grid_style(grid_style);

        self
    }

    /// Set the style of the subgrid.
    pub fn set_subgrid_style(&mut self, subgrid_style: ShapeStyle) {
        self.chart
            .set_data(|config| config.subgrid_style = subgrid_style)
    }

    #[inline]
    /// Set the style of the subgrid. Consumes self.
    pub fn subgrid_style(mut self, subgrid_style: ShapeStyle) -> Self {
        self.set_subgrid_style(subgrid_style);

        self
    }

    /// Set the style of the axes.
    pub fn set_axes_style(&mut self, axes_style: ShapeStyle) {
        self.chart.set_data(|config| config.axes_style = axes_style)
    }

    #[inline]
    /// Set the style of the plotted line. Consumes self.
    pub fn axes_style(mut self, axes_style: ShapeStyle) -> Self {
        self.set_axes_style(axes_style);

        self
    }

    /// Set the text color of the chart.
    pub fn set_text_color<T>(&mut self, color: T)
    where
        T: Into<RGBAColor>,
    {
        let color: RGBAColor = color.into();

        self.chart.set_data(|config| config.text_color = color)
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

    /// Set the background color of the chart.
    pub fn set_background_color<T>(&mut self, color: T)
    where
        T: Into<RGBAColor>,
    {
        let color: RGBAColor = color.into();

        self.chart
            .set_data(|config| config.background_color = color)
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
        self.chart.set_data(|config| config.ratio = ratio)
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
        if let Some(_) = self.playback_start {
            let time = self.current_time();

            let time_index = match self
                .times
                .binary_search_by(|probe| probe.partial_cmp(&time).unwrap_or(Ordering::Equal))
            {
                Ok(index) => index,
                Err(index) => self.points.len().min(index),
            };

            // The time index is always a valid index, so ensure the range is inclusive
            let points = &self.points[..=time_index];
            let range = self.ranges[time_index].clone();

            self.chart.set_data(|config| {
                config.points = points.into();
                config.range = range;
            });
        }

        self.chart.draw(ui);
    }

    #[inline]
    /// Start/enable playback of the chart.
    pub fn start_playback(&mut self) {
        self.playback_start = Some(Instant::now());
        self.pause_start = None;
    }

    #[inline]
    /// Stop/disable playback of the chart.
    pub fn stop_playback(&mut self) {
        self.playback_start = None;
        self.pause_start = None;
    }

    /// Toggle playback of the chart.
    pub fn toggle_playback(&mut self) {
        match self.playback_start {
            Some(playback_start) => match self.pause_start {
                Some(pause_start) => {
                    let delta = Instant::now().duration_since(pause_start);

                    self.pause_start = None;
                    self.playback_start = Some(playback_start + delta);
                }
                None => self.pause_start = Some(Instant::now()),
            },

            None => {
                self.start_playback();
            }
        }
    }

    #[inline]
    /// Return true if playback is currently enabled & underway.
    pub fn is_playing(&self) -> bool {
        self.playback_start != None && self.pause_start == None
    }

    #[inline]
    /// Return the time the chart starts at when playback is enabled.
    pub fn start_time(&self) -> f32 {
        let time_start = *self.times.first().unwrap();

        time_start
    }

    /// Return the current time to be animated when playback is enabled.
    pub fn current_time(&mut self) -> f32 {
        if let Some(playback_start) = self.playback_start {
            let now = Instant::now();

            let time_start = self.start_time();
            let time_end = self.end_time();

            let base_delta = time_end - time_start;

            // Ensure deltas are over 10us, otherwise they can cause overflows
            // in the plotters library
            let current_delta = MIN_DELTA
                + self.playback_speed
                    * match self.pause_start {
                        Some(pause_start) => {
                            pause_start.duration_since(playback_start).as_secs_f32()
                        }
                        None => now.duration_since(playback_start).as_secs_f32(),
                    };

            match base_delta > current_delta {
                true => current_delta + time_start,
                false => {
                    self.playback_start = None;

                    time_end
                }
            }
        } else {
            self.start_time()
        }
    }

    #[inline]
    /// Return the time the chart finished animating at when playback is enabled.
    pub fn end_time(&self) -> f32 {
        let time_end = *self.times.last().unwrap();

        time_end
    }

    #[inline]
    /// Return the speed the chart is animated at.
    pub fn get_playback_speed(&self) -> f32 {
        self.playback_speed
    }
}
