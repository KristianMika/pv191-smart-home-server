use super::models::HumidityTemperatureMeasurement;
use super::{SensorError, DHT11_PIN};
use error_stack::{IntoReport, Report, Result, ResultExt};
use rppal::gpio::{Gpio, Mode};
use rppal::hal::Delay;
use rppal_dht11::Dht11;

/// Can execute measurements using connected sensors
pub struct Sampler {
    /// DHT11 sensor for humidity and temperature
    dht11: Dht11,
}

impl Sampler {
    /// Creates a new instances of `Self`
    pub fn new() -> Result<Self, SensorError> {
        Ok(Self {
            dht11: Self::init_dht11(DHT11_PIN).attach_printable("Coudln't init dht11")?,
        })
    }

    /// Reads temperature and humidity using the connected sensors
    pub fn read_humidity_temperature(
        &mut self,
    ) -> Result<HumidityTemperatureMeasurement, SensorError> {
        match self.dht11.perform_measurement(&mut Delay::new()) {
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
}
