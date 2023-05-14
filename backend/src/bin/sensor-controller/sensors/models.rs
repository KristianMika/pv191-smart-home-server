use common::server_repo::postgres_server_repo::models::NewMeasurementStore;
use rppal_dht11::Measurement;
use serde::Serialize;

/// A temperature and humidity measurement
#[derive(Debug)]
pub struct HumidityTemperatureMeasurement {
    /// Relative humidity
    pub humidity: u32,
    /// Temperature in °C
    pub temperature: f32,
}

impl From<Measurement> for HumidityTemperatureMeasurement {
    fn from(value: Measurement) -> Self {
        Self {
            humidity: value.humidity as u32 / 10,
            temperature: value.temperature as f32 / 10.0,
        }
    }
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
