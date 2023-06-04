mod cli;
mod display_printer;
mod error;
mod sensors;

use chrono::{Duration, Local};
use clap::Parser;
use cli::Args;
use common::server_repo::{postgres_server_repo::PostgresServerRepo, ServerRepo};
use display_printer::{DisplayPrinter, Ssd1306Printer};
use dotenvy::dotenv;
use error::ControllerError;
use error_stack::ResultExt;
use sensors::sampler::{AirSensorSampler, SensorSampler};
use std::{env, io, time::Duration as StdDuration};
use tokio::time;

const DATABASE_URL_ENV: &str = "DATABASE_URL";

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let args = Args::parse();

    let db_url =
        env::var(DATABASE_URL_ENV).unwrap_or_else(|_| panic!("{} must be set", DATABASE_URL_ENV));

    let repo = PostgresServerRepo::from_url(&db_url).unwrap();
    let mut sampler = AirSensorSampler::new(args.get_dht11_pin(), args.get_voc_i2c_dev()).unwrap();
    let mut display_printer = Ssd1306Printer::new(args.get_display_i2c_dev()).unwrap();
    let mut interval = time::interval(StdDuration::from_secs(args.get_periodic_sampling_seconds()));

    let mut tick_counter = 0_u32;
    loop {
        tick_counter = tick_counter.wrapping_add(1);

        interval.tick().await;
        match on_tick(&repo, &mut sampler, &mut display_printer) {
            Ok(_) => {}
            Err(err) => {
                log::error!("{:?}", err)
            }
        };

        if is_nth_tick(tick_counter as u64, args.get_remove_measurements_after()) {
            let remove_measurements_to =
                Local::now() - Duration::days(args.get_keep_measurements_days() as i64);
            if let Err(err) = repo.delete_measurements_older_than(remove_measurements_to) {
                log::error!("Couldn't delete measurements: {err:?}");
            }
        }
    }
}

fn is_nth_tick(tick: u64, n: u64) -> bool {
    tick % n == 0
}

/// Execute tasks at the end of each interval
///
/// 1. Sample new values
/// 2. Store values into DB
/// 3. Update display printer
fn on_tick(
    repo: &impl ServerRepo,
    sampler: &mut impl SensorSampler,
    display_printer: &mut impl DisplayPrinter,
) -> error_stack::Result<(), ControllerError> {
    let sample = sampler
        .perfom_measurement()
        .change_context(ControllerError)
        .attach_printable("Couldn't perform measurement")?;
    repo.store_measurement(sample)
        .change_context(ControllerError)
        .attach_printable("Couldn't store measurement")?;
    display_printer
        .print_measurement(sample.into())
        .change_context(ControllerError)
        .attach_printable("Coudln't print measurement")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use common::server_repo::{postgres_server_repo::models::NewMeasurementStore, MockServerRepo};
    use mockall::Sequence;

    use crate::{
        display_printer::MockDisplayPrinter, error::ControllerError, on_tick,
        sensors::sampler::MockSensorSampler,
    };

    #[test]
    fn given_sampler_returns_measurements_measurement_is_stored_and_printed(
    ) -> error_stack::Result<(), ControllerError> {
        let mut seq = Sequence::new();

        let mut mock_sensor_sampler = MockSensorSampler::new();
        let mut mock_repo = MockServerRepo::new();
        let mut mock_display_printer = MockDisplayPrinter::new();

        let mocked_measurement = NewMeasurementStore {
            temperature: Some(16.8),
            humidity: Some(58),
            voc_index: None,
            measurement_time: None,
        };
        mock_sensor_sampler
            .expect_perfom_measurement()
            .once()
            .in_sequence(&mut seq)
            .returning(move || Ok(mocked_measurement));

        mock_repo
            .expect_store_measurement()
            .once()
            .in_sequence(&mut seq)
            .returning(move |_| Ok(()));

        mock_display_printer
            .expect_print_measurement()
            .once()
            .in_sequence(&mut seq)
            .returning(move |_| Ok(()));

        on_tick(
            &mock_repo,
            &mut mock_sensor_sampler,
            &mut mock_display_printer,
        )
    }
}
