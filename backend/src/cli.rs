use clap::{arg, command, Parser};

/// A simple server application for Raspberry PI that measures temperature, humidity, and VOC index, and provides the data via a web interface.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// I2C device to use for the display
    #[arg(long, default_value_t = String::from("/dev/i2c-1"))]
    display_i2c_dev: String,

    /// I2C device to use for the VOC sensor
    #[arg(long, default_value_t = String::from("/dev/i2c-2"))]
    voc_i2c_dev: String,

    /// DHT11 pin
    #[arg(long, default_value_t = 23)]
    dht11_pin: u8,

    /// Measurement period in seconds
    #[arg(long, default_value_t = 30)]
    periodic_sampling_seconds: u64,
}

impl Args {
    pub fn get_display_i2c_dev(&self) -> &str {
        &self.display_i2c_dev
    }

    pub fn get_voc_i2c_dev(&self) -> &str {
        &self.voc_i2c_dev
    }

    pub fn get_dht11_pin(&self) -> u8 {
        self.dht11_pin
    }

    pub fn get_periodic_sampling_seconds(&self) -> u64 {
        self.periodic_sampling_seconds
    }
}
