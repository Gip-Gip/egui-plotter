# Changelog

## Breaking Changes & Fixes

### 0.3.0 -> 0.4.0

 * `XyTimeData` and `TimeData` are now locked behind the feature `timechart`.
    * Specify feature `timechart` when including in your Cargo.toml to use these built in charts.
 * `Chart` type now generic and mutable
    * Specify chart data type when initializing, or if it has none create a `Chart::<()>::new()`
    * Accessing the chart's data no longer requires a `.downcast` and is directly accessable

## 0.3.0

 * Added premade charts
    * Added TimeChart
    * Added XyTimeChart

## 0.2.1

 * Fixed incompatablities with git version of egui

## 0.2.0
 * Added Chart type to allow for easier interactive charts.
 * Added examples for the Chart type.
 * Extended backend for allowing the modificaiton of drawing area scale and offset.
 * Improved documentation.

## 0.1.1
 * Made 3d plot example interactive
 * Added demo GIF to readme
