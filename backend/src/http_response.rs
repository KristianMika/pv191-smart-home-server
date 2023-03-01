use serde::Serialize;

/// A struct used to respond to a single measurement request
#[derive(Serialize)]
pub struct MeasurementResponse {
    /// Temperature in °C
    pub temperature: f32,
    /// Pressure in hPa
    pub pressure: u32,
    /// Humidity in %
    pub humidity: u32,
}
