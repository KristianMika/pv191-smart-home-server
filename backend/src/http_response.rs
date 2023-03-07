use chrono::{DateTime, Local};
use serde::Serialize;

/// A struct used to respond to a single measurement request
#[derive(Serialize)]
pub struct MeasurementResponse {
    pub measurement: MeasurementData,
    pub measurement_time: DateTime<Local>,
}

#[derive(Serialize)]
pub struct MeasurementData {
    /// Temperature in °C
    pub temperature: f32,
    /// Humidity in %
    pub humidity: u32,
    /// VOC Index in range [0,500]
    pub voc_index: u32,
}
