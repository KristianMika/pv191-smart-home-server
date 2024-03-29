use clap::{arg, command, Parser};
use clap_num::number_range;

const MEASUREMENT_PERIOD_MIN_SECONDS: u64 = 30;
const MEASUREMENT_PERIOD_MAX_SECONDS: u64 = 5 * 60;
/// A simple server application for Raspberry PI that measures temperature, humidity, and VOC index, and provides the data via a web interface.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// I2C device to use for the display
    #[arg(long, default_value_t = String::from("/dev/i2c-1"))]
    display_i2c_dev: String,

    /// I2C device to use for the VOC sensor
    #[arg(long, default_value_t = String::from("/dev/i2c-1"))]
    voc_i2c_dev: String,

    /// I2C device to use for the humidity and temperature sensor
    #[arg(long, default_value_t = String::from("/dev/i2c-1"))]
    humidity_temperature_i2c_dev: String,

    /// DHT11 pin
    #[arg(long, default_value_t = 25)]
    dht11_pin: u8,

    /// Measurement period in seconds in range [30, 300]
    #[arg(long, default_value_t = 30, value_parser=measurement_period_seconds_parser)]
    periodic_sampling_seconds: u64,

    /// The number of periods to wait for old measurements removal
    #[arg(long, default_value_t = 200)]
    remove_measurements_after: u64,

    /// How many days of measurements to keep
    #[arg(long, default_value_t = 2)]
    keep_measurements_days: u32,
}

fn measurement_period_seconds_parser(period: &str) -> Result<u64, String> {
    number_range(
        period,
        MEASUREMENT_PERIOD_MIN_SECONDS,
        MEASUREMENT_PERIOD_MAX_SECONDS,
    )
}

impl Args {
    pub fn get_display_i2c_dev(&self) -> &str {
        &self.display_i2c_dev
    }

    pub fn get_voc_i2c_dev(&self) -> &str {
        &self.voc_i2c_dev
    }

    pub fn get_humidity_temperature_i2c_dev(&self) -> &str {
        &self.humidity_temperature_i2c_dev
    }

    pub fn get_periodic_sampling_seconds(&self) -> u64 {
        self.periodic_sampling_seconds
    }

    pub fn get_remove_measurements_after(&self) -> u64 {
        self.remove_measurements_after
    }

    pub fn get_keep_measurements_days(&self) -> u32 {
        self.keep_measurements_days
    }
}
