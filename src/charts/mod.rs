//! Various type of premade charts.

#[cfg(feature = "timechart")]
mod timedata;
#[cfg(feature = "timechart")]
mod xytime;

#[cfg(feature = "timechart")]
pub use timedata::TimeData;
#[cfg(feature = "timechart")]
pub use xytime::XyTimeData;
