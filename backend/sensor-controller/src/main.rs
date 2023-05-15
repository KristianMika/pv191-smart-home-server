mod cli;
mod display_printer;
mod error;
mod sensors;

use clap::Parser;
use cli::Args;
use common::server_repo::{postgres_server_repo::PostgresServerRepo, ServerRepo};
use display_printer::DisplayPrinter;
use dotenvy::dotenv;
use error::ControllerError;
use error_stack::ResultExt;
use sensors::sampler::Sampler;
use std::{env, io, time::Duration};
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
    let mut sampler = Sampler::new(args.get_dht11_pin(), args.get_voc_i2c_dev()).unwrap();
    let mut display_printer = DisplayPrinter::new(args.get_display_i2c_dev()).unwrap();
    let mut interval = time::interval(Duration::from_secs(args.get_periodic_sampling_seconds()));

    loop {
        interval.tick().await;
        match on_tick(&repo, &mut sampler, &mut display_printer).await {
            Ok(_) => {}
            Err(err) => {
                log::error!("{:?}", err)
            }
        };
    }
}

/// Execute tasks at the end of each interval
///
/// 1. Sample new values
/// 2. Store values into DB
/// 3. Update display printer
async fn on_tick(
    repo: &PostgresServerRepo,
    sampler: &mut Sampler,
    display_printer: &mut DisplayPrinter,
) -> error_stack::Result<(), ControllerError> {
    let sample = sampler
        .perfom_measurement()
        .change_context(ControllerError)
        .attach_printable("Couldn't perform measurement")?;
    repo.store_measurement(sample.clone())
        .change_context(ControllerError)
        .attach_printable("Couldn't store measurement")?;
    display_printer
        .print_measurement(sample.into())
        .change_context(ControllerError)
        .attach_printable("Coudln't print measurement")?;

    Ok(())
}
