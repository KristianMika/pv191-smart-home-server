use rppal_dht11::Measurement;

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
