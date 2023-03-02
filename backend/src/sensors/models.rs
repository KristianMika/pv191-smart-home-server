use rppal_dht11::Measurement;

/// A temperature and humidity measurement
#[derive(Debug)]
pub struct HumidityTemperatureMeasurement {
    /// Relative humidity
    humidity: f32,
    /// Temperature in °C
    temperature: f32,
}

impl From<Measurement> for HumidityTemperatureMeasurement {
    fn from(value: Measurement) -> Self {
        Self {
            humidity: value.humidity as f32 / 10.0,
            temperature: value.temperature as f32 / 10.0,
        }
    }
}
