use common::server_repo::postgres_server_repo::models::NewMeasurementStore;
use error_stack::{IntoReport, Report, Result, ResultExt};
use hal::{Delay, I2cdev};
use linux_embedded_hal as hal;
use log::error;
use mockall::automock;
use sensor_temp_humidity_sht40::{I2CAddr, Precision, SHT40Driver, TempUnit};
use sgp40::Sgp40;

use super::error::SensorError;
use super::models::HumidityTemperatureMeasurement;

const SGP40_I2C_ADDRESS: u8 = 0x59;
const VOC_INIT_REPETITION_COUNT: usize = 50;
const SGP_MIN_INITIALIZED_VALUE: u16 = 5;

type VocIndex = u16;

#[automock]
pub trait SensorSampler {
    fn perfom_measurement(&mut self) -> error_stack::Result<NewMeasurementStore, SensorError>;
}
/// Can execute measurements using connected sensors
pub struct AirSensorSampler {
    /// SHT40 sensor for humidity and temperature
    sht40: SHT40Driver<I2cdev, Delay>,
    /// Sgp40 sensor for VOC index
    sgp40: Sgp40<I2cdev, Delay>,
}

impl AirSensorSampler {
    /// Creates a new instances of `Self`
    pub fn new(voc_i2c_dev: &str, humid_temp_i2c_dev: &str) -> Result<Self, SensorError> {
        let sampler = Self {
            sht40: Self::init_sht40(humid_temp_i2c_dev).attach_printable("Coudln't init dht11")?,
            sgp40: Self::init_sgp40(voc_i2c_dev).attach_printable("Couldn't init sgp40")?,
        };
        Ok(sampler)
    }

    /// Reads temperature and humidity using the connected sensors
    pub fn read_humidity_temperature(
        &mut self,
    ) -> Result<HumidityTemperatureMeasurement, SensorError> {
        match self
            .sht40
            .get_temp_and_rh(Precision::High, TempUnit::MilliDegreesCelsius)
        {
            Ok(m) => Ok(m.into()),
            Err(err) => Err(Report::new(SensorError).attach_printable(format!(
                "Couldn't perform humidity and temperature measurement: {:?}",
                err
            ))),
        }
    }

    fn init_sht40(i2c_dev_name: &str) -> Result<SHT40Driver<I2cdev, Delay>, SensorError> {
        let i2c_dev = I2cdev::new(i2c_dev_name)
            .into_report()
            .change_context(SensorError)
            .attach_printable("Couldn't create i2c dev")?;
        Ok(SHT40Driver::new(i2c_dev, I2CAddr::SHT4x_A, Delay))
    }

    fn init_sgp40(device_name: &str) -> Result<Sgp40<I2cdev, Delay>, SensorError> {
        let i2c_dev = I2cdev::new(device_name)
            .into_report()
            .change_context(SensorError)
            .attach_printable(format!("Couldn't get I2C device {}", device_name))?;
        Ok(Sgp40::new(i2c_dev, SGP40_I2C_ADDRESS, Delay))
    }

    fn measure_voc_index(
        &mut self,
        temperature: Option<f32>,
        humidity: Option<u32>,
    ) -> error_stack::Result<VocIndex, SensorError> {
        if temperature.is_none() || humidity.is_none() {
            return self.sgp40.measure_voc_index().map_err(|err| {
                error_stack::Report::new(SensorError)
                    .attach_printable(format!("Couldn't perform VOC measurement: {:?}", err))
            });
        }
        let temperature_in_mili = (temperature.unwrap() * 1000.0) as i16;
        let humidity_in_mili = (humidity.unwrap() * 1000) as u16;
        self.sgp40
            .measure_voc_index_with_rht(humidity_in_mili, temperature_in_mili)
            .map_err(|err| {
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

        // SGP40 may need a few more reads to init the algorithm
        for _ in 0..VOC_INIT_REPETITION_COUNT {
            match self.measure_voc_index(temperature, humidity) {
                Ok(sample) => {
                    voc_index = Some(sample);
                    if sample >= SGP_MIN_INITIALIZED_VALUE {
                        break;
                    }
                }
                Err(err) => error!("{:?}", err),
            };
        }

        Ok(NewMeasurementStore {
            temperature,
            humidity: humidity.map(|val| val as i32),
            voc_index: voc_index.map(|val| val as i32),
            measurement_time: None,
        })
    }
}
