use common::server_repo::postgres_server_repo::models::NewMeasurementStore;
use sensor_temp_humidity_sht40::Measurement;
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
            humidity: value.rel_hum_pcm / 1000,
            temperature: (value.temp as f32 / 1000.0),
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
