use std::{error::Error, fmt};

/// An error that ocurred when interracting with a sensor
#[derive(Debug)]
pub struct SensorError;

impl Error for SensorError {}
impl fmt::Display for SensorError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("could not interract with the sensor")
    }
}
