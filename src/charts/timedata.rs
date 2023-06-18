//! Chart with data on the Y and time on the X axis

use std::{
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
        Color, FontDesc, ShapeStyle,
    },
};
use plotters_backend::{FontFamily, FontStyle};

use crate::{charts::XyTimeData, Chart, MouseConfig};

pub struct TimeData {
    chart: XyTimeData,
}

impl TimeData {
    pub fn new(points: &[(f32, f32)], unit: &str, caption: &str) -> Self {
        let points: Vec<(f32, f32, f32)> = points
            .into_iter()
            .map(|(x, time)| (*x, *time, *time))
            .collect();

        let chart = XyTimeData::new(&points, unit, "seconds", caption);

        Self { chart }
    }

    #[inline]
    pub fn set_time(&mut self, time: f32) {
        self.chart.set_time(time)
    }

    #[inline]
    pub fn time(mut self, time: f32) -> Self {
        self.set_time(time);

        self
    }

    #[inline]
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.chart.set_playback_speed(speed)
    }

    #[inline]
    pub fn playback_speed(mut self, speed: f32) -> Self {
        self.set_playback_speed(speed);

        self
    }

    pub fn draw(&mut self, ui: &Ui) {
        self.chart.draw(ui)
    }

    #[inline]
    pub fn start_playback(&mut self) {
        self.chart.start_playback()
    }

    #[inline]
    pub fn stop_playback(&mut self) {
        self.chart.stop_playback()
    }

    pub fn toggle_playback(&mut self) {
        self.chart.toggle_playback()
    }

    #[inline]
    pub fn is_playing(&self) -> bool {
        self.chart.is_playing()
    }

    #[inline]
    pub fn start_time(&self) -> f32 {
        self.chart.start_time()
    }

    #[inline]
    pub fn current_time(&mut self) -> f32 {
        self.chart.current_time()
    }

    #[inline]
    pub fn end_time(&self) -> f32 {
        self.chart.end_time()
    }

    #[inline]
    pub fn get_playback_speed(&self) -> f32 {
        self.chart.get_playback_speed()
    }
}
