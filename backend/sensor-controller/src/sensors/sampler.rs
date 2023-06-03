use common::server_repo::postgres_server_repo::models::NewMeasurementStore;
use error_stack::{IntoReport, Report, Result, ResultExt};
use linux_embedded_hal::I2cdev;
use log::error;
use mockall::automock;
use pyo3::Python;
use rppal::gpio::{Gpio, Mode};
use rppal::hal::Delay;
use sgp40::Sgp40;
use std::collections::HashMap;

use pyo3::{
    types::{PyModule, PyTuple},
    Py, PyAny, PyObject,
};

use super::error::SensorError;
use super::models::HumidityTemperatureMeasurement;

const SGP40_I2C_ADDRESS: u8 = 0x59;
type VocIndex = u16;

#[derive(Clone, Debug)]
struct PgipioDhtResponse {
    temp_c: i32,
    temp_f: f32,
    humidity: u32,
    valid: bool,
}

#[automock]
pub trait SensorSampler {
    fn perfom_measurement(&mut self) -> error_stack::Result<NewMeasurementStore, SensorError>;
}
/// Can execute measurements using connected sensors
pub struct AirSensorSampler {
    /// DHT11 sensor for humidity and temperature
    /// Sgp40 sensor for VOC index
    sgp40: Sgp40<I2cdev, Delay>,
}

impl AirSensorSampler {
    /// Creates a new instances of `Self`
    pub fn new(dht11_pin: u8, voc_i2c_dev: &str) -> Result<Self, SensorError> {
        Self::init_dht11(dht11_pin).attach_printable("Coudln't init dht11")?;
        let sampler = Self {
            sgp40: Self::init_sgp40(voc_i2c_dev)?,
        };
        Ok(sampler)
    }

    /// Reads temperature and humidity using the connected sensors
    pub fn read_humidity_temperature(
        &mut self,
    ) -> Result<HumidityTemperatureMeasurement, SensorError> {
        Python::with_gil(|py| {
            let read_sensor: Py<PyAny> = PyModule::from_code(
                py,
                "
def read_sensor(*args, **kwargs):
    from pigpio_dht import DHT11, DHT22

    gpio = args[0]
    #sensor = DHT22(gpio)

    return {'temp_c': 20, 'temp_f': 68.0, 'humidity': 35, 'valid': True}        
    return sensor.read(3)
    ",
                "",
                "",
            )
            .unwrap()
            .getattr("read_sensor")
            .unwrap()
            .into();
            let gpio = 25;
            let args = PyTuple::new(py, &[gpio]);

            let value: HashMap<String, PyObject> =
                read_sensor.call1(py, args).unwrap().extract(py).unwrap();

            let res = PgipioDhtResponse {
                temp_c: value.get("temp_c").unwrap().extract(py).unwrap(),
                temp_f: value.get("temp_f").unwrap().extract(py).unwrap(),
                humidity: value.get("humidity").unwrap().extract(py).unwrap(),
                valid: value.get("valid").unwrap().extract(py).unwrap(),
            };
            if !res.valid {
                return Err(Report::new(SensorError));
            }
            return Ok(HumidityTemperatureMeasurement {
                humidity: res.humidity,
                temperature: res.temp_c as f32,
            });
        })
    }

    /// Initializes a DHT11 sensor
    fn init_dht11(gpio_pin: u8) -> Result<(), SensorError> {
        pyo3::prepare_freethreaded_python();

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

        Ok(())
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
