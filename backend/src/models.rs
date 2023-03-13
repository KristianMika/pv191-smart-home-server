use chrono::{DateTime, Local};
use serde::Serialize;

use crate::server_repo::postgres_server_repo::models::NewMeasurementStore;

/// A struct used to respond to a single measurement request
#[derive(Serialize)]
pub struct MeasurementResponse {
    pub measurement: MeasurementData,
    pub measurement_time: DateTime<Local>,
}

#[derive(Serialize)]
pub struct MeasurementData {
    /// Temperature in °C
    pub temperature: Option<f32>,
    /// Humidity in %
    pub humidity: Option<u32>,
    /// VOC Index in range [0,500]
    pub voc_index: Option<u32>,
}

impl From<NewMeasurementStore> for MeasurementData {
    fn from(value: NewMeasurementStore) -> Self {
        MeasurementData {
            temperature: value.temperature,
            humidity: value.humidity.map(|num| num as u32),
            voc_index: value.voc_index.map(|num| num as u32),
        }
    }
}
