//! Chart with data on the Y and time on the X axis

use std::{
    any::Any,
    cmp::Ordering,
    ops::Range,
    time::{Duration, Instant},
};

use egui::Ui;
use plotters::{
    prelude::ChartBuilder,
    series::LineSeries,
    style::{
        full_palette::{GREY, RED_900},
        Color, FontDesc, RGBAColor, ShapeStyle, TextStyle, BLACK,
    },
};
use plotters_backend::{FontFamily, FontStyle};

use crate::{Chart, MouseConfig};

#[derive(Debug, Clone)]
struct TimeConfig {
    points: Vec<(f32, f32)>,
    range: (Range<f32>, Range<f32>),
    unit: String,
    caption: String,
}

pub struct TimeData {
    config: TimeConfig,
    playback_start: Option<Instant>,
    chart: Chart,
}

impl TimeData {
    pub fn new(points: &[(f32, f32)], unit: &str, caption: &str) -> Self {
        let (data_min, _) = *points
            .iter()
            .min_by(|a, b| {
                let (data_a, _ta) = a;
                let (data_b, _tb) = b;

                data_a.partial_cmp(data_b).unwrap_or(Ordering::Equal)
            })
            .unwrap();

        let (data_max, _) = *points
            .iter()
            .max_by(|a, b| {
                let (data_a, _ta) = a;
                let (data_b, _tb) = b;

                data_a.partial_cmp(data_b).unwrap_or(Ordering::Equal)
            })
            .unwrap();

        let (_, time_min) = *points
            .iter()
            .min_by(|a, b| {
                let (_da, time_a) = a;
                let (_db, time_b) = b;

                time_a.partial_cmp(time_b).unwrap_or(Ordering::Equal)
            })
            .unwrap();

        let (_, time_max) = *points
            .iter()
            .max_by(|a, b| {
                let (_da, time_a) = a;
                let (_db, time_b) = b;

                time_a.partial_cmp(time_b).unwrap_or(Ordering::Equal)
            })
            .unwrap();

        let data_range = data_min..data_max;
        let time_range = time_min..time_max;

        // Make the unit string vertical
        let unit: String = unit.split("").map(|c| format!("{}\n", c)).collect();

        let config = TimeConfig {
            points: points.to_vec(),
            range: (data_range, time_range),
            unit,
            caption: caption.to_string(),
        };

        let chart = Chart::new()
            .mouse(MouseConfig::enabled())
            .data(Box::new(config.clone()))
            .builder_cb(Box::new(|area, _t, data| {
                let data: &TimeConfig = data.as_ref().unwrap().downcast_ref().unwrap();

                let (x_range, y_range) = data.range.clone();

                let font_style = FontStyle::Normal;
                let font_family = FontFamily::Monospace;
                let font_size = 10;

                let font_desc = FontDesc::new(font_family, font_size as f64, font_style);

                let grid_style = ShapeStyle {
                    color: GREY.to_rgba(),
                    filled: false,
                    stroke_width: 1,
                };

                let line_style = ShapeStyle {
                    color: RED_900.to_rgba(),
                    filled: false,
                    stroke_width: 2,
                };

                let mut chart = ChartBuilder::on(area)
                    .margin(25)
                    .caption(data.caption.clone(), font_desc.clone())
                    .x_label_area_size(25)
                    .y_label_area_size(25)
                    .build_cartesian_2d(x_range, y_range)
                    .unwrap();

                chart
                    .configure_mesh()
                    .label_style(font_desc.clone())
                    .light_line_style(grid_style)
                    .x_desc("seconds")
                    .set_all_tick_mark_size(4)
                    .y_desc(&data.unit)
                    .draw()
                    .unwrap();

                chart
                    .draw_series(LineSeries::new(data.points.clone(), line_style))
                    .unwrap();
            }));

        Self {
            config,
            playback_start: None,
            chart,
        }
    }

    pub fn draw(&mut self, ui: &Ui) {
        if let Some(playback_start) = self.playback_start {
            let (mut x_range, y_range) = self.config.range.clone();

            let now = Instant::now();

            let base_delta = x_range.end - x_range.start;
            let current_delta = now.duration_since(playback_start).as_secs_f32();

            let end = match base_delta > current_delta {
                true => current_delta + x_range.start,
                false => {
                    self.playback_start = None;

                    x_range.end
                }
            };

            x_range = x_range.start..end;

            let mut current_config = self.config.clone();

            current_config.range = (x_range, y_range);

            self.chart.set_data(Box::new(current_config));
        }

        self.chart.draw(ui);
    }

    #[inline]
    pub fn start_playback(&mut self) {
        self.playback_start = Some(Instant::now())
    }
}
