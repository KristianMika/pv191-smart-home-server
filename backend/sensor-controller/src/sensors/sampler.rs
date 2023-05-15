use common::server_repo::postgres_server_repo::models::NewMeasurementStore;
use error_stack::{IntoReport, Report, Result, ResultExt};
use linux_embedded_hal::I2cdev;
use log::error;
use rppal::gpio::{Gpio, Mode};
use rppal::hal::Delay;
use rppal_dht11::Dht11;
use sgp40::Sgp40;

use super::error::SensorError;
use super::models::HumidityTemperatureMeasurement;

const SGP40_I2C_ADDRESS: u8 = 0x59;
type VocIndex = u16;

pub trait SensorSampler {
    fn perfom_measurement(&mut self) -> error_stack::Result<NewMeasurementStore, SensorError>;
}
/// Can execute measurements using connected sensors
pub struct AirSensorSampler {
    /// DHT11 sensor for humidity and temperature
    dht11: Dht11,
    /// Sgp40 sensor for VOC index
    sgp40: Sgp40<I2cdev, Delay>,
}

impl AirSensorSampler {
    /// Creates a new instances of `Self`
    pub fn new(dht11_pin: u8, voc_i2c_dev: &str) -> Result<Self, SensorError> {
        let sampler = Self {
            dht11: Self::init_dht11(dht11_pin).attach_printable("Coudln't init dht11")?,
            sgp40: Self::init_sgp40(voc_i2c_dev)?,
        };
        Ok(sampler)
    }

    /// Reads temperature and humidity using the connected sensors
    pub fn read_humidity_temperature(
        &mut self,
    ) -> Result<HumidityTemperatureMeasurement, SensorError> {
        match self
            .dht11
            .perform_measurement_with_retries(&mut Delay::new(), 5)
        {
            Ok(m) => Ok(m.into()),
            Err(err) => Err(Report::new(SensorError).attach_printable(format!(
                "Couldn't perform humidity and temperature measurement: {:?}",
                err
            ))),
        }
    }

    /// Initializes a DHT11 sensor
    fn init_dht11(gpio_pin: u8) -> Result<Dht11, SensorError> {
        let gpio = Gpio::new()
            .into_report()
            .change_context(SensorError)
            .attach_printable("Couldn't init gpio")?;
        let pin = gpio
            .get(gpio_pin)
            .into_report()
            .change_context(SensorError)
            .attach_printable(format!("Couldn't get pin #{}", gpio_pin))?
            .into_io(Mode::Output);
        Ok(Dht11::new(pin))
    }

    fn init_sgp40(device_name: &str) -> Result<Sgp40<I2cdev, Delay>, SensorError> {
        let i2c_dev = I2cdev::new(device_name)
            .into_report()
            .change_context(SensorError)
            .attach_printable(format!("Couldn't get I2C device {}", device_name))?;
        Ok(Sgp40::new(i2c_dev, SGP40_I2C_ADDRESS, Delay))
    }

    fn measure_voc_index(&mut self) -> error_stack::Result<VocIndex, SensorError> {
        self.sgp40.measure_voc_index().map_err(|err| {
            error_stack::Report::new(SensorError)
                .attach_printable(format!("Couldn't perform VOC measurement: {:?}", err))
        })
    }
}

impl SensorSampler for AirSensorSampler {
    fn perfom_measurement(&mut self) -> error_stack::Result<NewMeasurementStore, SensorError> {
        let mut temperature = None;
        let mut humidity = None;
        let mut voc_index = None;

        match self.read_humidity_temperature() {
            Ok(sample) => {
                temperature = Some(sample.temperature);
                humidity = Some(sample.humidity)
            }
            Err(err) => {
                error!("{:?}", err)
            }
        };

        match self.measure_voc_index() {
            Ok(sample) => voc_index = Some(sample),
            Err(err) => error!("{:?}", err),
        };

        Ok(NewMeasurementStore {
            temperature,
            humidity: humidity.map(|val| val as i32),
            voc_index: voc_index.map(|val| val as i32),
            measurement_time: None,
        })
    }
}
