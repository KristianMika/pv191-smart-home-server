extern crate linux_embedded_hal as hal;

use crate::sensors::{error::SensorError, models::MeasurementData};
use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use error_stack::{IntoReport, Report, ResultExt};
use hal::I2cdev;
use local_ip_address::local_ip;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};
use std::{error::Error, fmt};

pub struct DisplayPrinter {
    display: Ssd1306<
        I2CInterface<I2cdev>,
        ssd1306::prelude::DisplaySize128x64,
        BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
    >,
    xpos: u32,
    ypos: u32,
}

pub fn get_voc_evaluation(voc: u32) -> error_stack::Result<String, SensorError> {
    Ok(match voc {
        0..=100 => "perfect".into(),
        101..=200 => "good".into(),
        201..=300 => "vent!".into(),
        301..=400 => "vent!!!".into(),
        401..=500 => "danger!".into(),
        _ => return Err(Report::new(SensorError).attach_printable(format!("Invalid VOC {}", voc))),
    })
}

impl DisplayPrinter {
    pub fn new(i2c_dev: &str) -> error_stack::Result<Self, DisplayPrinterError> {
        let i2c = I2cdev::new(i2c_dev)
            .into_report()
            .change_context(DisplayPrinterError)
            .attach_printable("Couldn't init i2c-1 device")?;

        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        if let Err(err) = display.init() {
            return Err(Report::new(DisplayPrinterError)
                .attach_printable(format!("Couldn't init display: {:?}", err)));
        };
        display.clear();
        if let Err(err) = display.flush() {
            return Err(Report::new(DisplayPrinterError)
                .attach_printable(format!("Coudln't flush display: {:?}", err)));
        };
        Ok(Self {
            display,
            xpos: 0,
            ypos: 0,
        })
    }

    pub fn writeln(
        &mut self,
        line: &str,
        text_style: MonoTextStyle<'static, BinaryColor>,
    ) -> error_stack::Result<(), DisplayPrinterError> {
        if let Err(err) = Text::with_baseline(
            line,
            Point::new(self.xpos as i32, self.ypos as i32),
            text_style,
            Baseline::Top,
        )
        .draw(&mut self.display)
        {
            return Err(Report::new(DisplayPrinterError)
                .attach_printable(format!("Couldn't print: {:?}", err)))?;
        };
        self.xpos = 0;
        self.ypos += text_style.font.character_size.height;
        if let Err(err) = self.display.flush() {
            return Err(Report::new(DisplayPrinterError)
                .attach_printable(format!("Couldn't flush display: {:?}", err)))?;
        };

        Ok(())
    }

    pub fn reset_position(&mut self) {
        self.xpos = 0;
        self.ypos = 0;
    }

    pub fn print_measurement(
        &mut self,
        measurement: MeasurementData,
    ) -> error_stack::Result<(), DisplayPrinterError> {
        let text_style = Self::get_default_text_style();
        self.clear_display()?;

        self.print_voc(measurement.voc_index, text_style)?;
        self.print_temperature(measurement.temperature, text_style)?;
        self.print_humidity(measurement.humidity, text_style)?;
        self.print_ip(text_style)
    }

    pub fn clear_display(&mut self) -> error_stack::Result<(), DisplayPrinterError> {
        self.reset_position();
        self.display.clear();
        if let Err(err) = self.display.flush() {
            return Err(Report::new(DisplayPrinterError))
                .attach_printable(format!("Couldn't clear and flush display: {:?}", err));
        }

        Ok(())
    }

    fn get_default_text_style() -> MonoTextStyle<'static, BinaryColor> {
        MonoTextStyleBuilder::new()
            .font(&FONT_8X13)
            .text_color(BinaryColor::On)
            .build()
    }

    fn print_voc(
        &mut self,
        voc: Option<u32>,
        text_style: MonoTextStyle<'static, BinaryColor>,
    ) -> error_stack::Result<(), DisplayPrinterError> {
        let to_print = match voc {
            Some(value) => {
                let voc_evaluation = get_voc_evaluation(value)
                    .change_context(DisplayPrinterError)
                    .attach_printable("Couldn't get VOC string evaluation")?;
                format!("{} {}", value, voc_evaluation)
            }
            None => "-".into(),
        };
        self.writeln(&format!("VOC: {}", to_print), text_style)
    }

    fn print_temperature(
        &mut self,
        temperature: Option<f32>,
        text_style: MonoTextStyle<'static, BinaryColor>,
    ) -> error_stack::Result<(), DisplayPrinterError> {
        let to_print = match temperature {
            Some(value) => format!("{:.1} C", value),
            None => "-".into(),
        };

        self.writeln(&format!("Temp: {}", to_print), text_style)
    }

    fn print_humidity(
        &mut self,
        humidity: Option<u32>,
        text_style: MonoTextStyle<'static, BinaryColor>,
    ) -> error_stack::Result<(), DisplayPrinterError> {
        let to_print = match humidity {
            Some(value) => format!("{}%", value),
            None => "-".into(),
        };

        self.writeln(&format!("Humid: {}", to_print), text_style)
    }

    fn print_ip(
        &mut self,
        text_style: MonoTextStyle<'static, BinaryColor>,
    ) -> error_stack::Result<(), DisplayPrinterError> {
        let ip_to_print = if let Ok(ip_address) = local_ip() {
            ip_address.to_string()
        } else {
            "-".into()
        };
        self.writeln(&ip_to_print, text_style)
    }
}

#[derive(Debug)]
pub struct DisplayPrinterError;

impl Error for DisplayPrinterError {}
impl fmt::Display for DisplayPrinterError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("an error with display ocurred")
    }
}
