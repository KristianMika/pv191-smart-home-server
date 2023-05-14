use clap::Parser;
use cli::Args;
use common::server_repo::{postgres_server_repo::PostgresServerRepo, ServerRepo};
use display_printer::DisplayPrinter;
use dotenvy::dotenv;
use error_stack::ResultExt;
use sensors::sampler::Sampler;
use std::{env, error::Error, fmt, io, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time};

mod cli;
mod display_printer;
mod sensors;

const DATABASE_URL_ENV: &str = "DATABASE_URL";

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let args = Args::parse();

    let db_url =
        env::var(DATABASE_URL_ENV).unwrap_or_else(|_| panic!("{} must be set", DATABASE_URL_ENV));

    let repo = Arc::new(PostgresServerRepo::from_url(&db_url).unwrap());
    let sampler = Arc::new(Mutex::new(
        Sampler::new(args.get_dht11_pin(), args.get_voc_i2c_dev()).unwrap(),
    ));
    let display_printer = Arc::new(Mutex::new(
        DisplayPrinter::new(args.get_display_i2c_dev()).unwrap(),
    ));

    let mut interval = time::interval(Duration::from_secs(args.get_periodic_sampling_seconds()));

    loop {
        interval.tick().await;
        match do_tick(&repo, &sampler, &display_printer).await {
            Ok(_) => {}
            Err(err) => {
                log::error!("{:?}", err)
            }
        };
    }
}

async fn do_tick(
    repo: &Arc<PostgresServerRepo>,
    sampler: &Arc<Mutex<Sampler>>,
    display_printer: &Arc<Mutex<DisplayPrinter>>,
) -> error_stack::Result<(), ServerError> {
    let sample = sampler
        .lock()
        .await
        .perfom_measurement()
        .change_context(ServerError)
        .attach_printable("Couldn't perform measurement")?;
    repo.store_measurement(sample.clone())
        .change_context(ServerError)
        .attach_printable("Couldn't store measurement")?;
    display_printer
        .lock()
        .await
        .print_measurement(sample.into())
        .change_context(ServerError)
        .attach_printable("Coudln't print measurement")?;

    Ok(())
}

#[derive(Debug)]
pub struct ServerError;

impl Error for ServerError {}
impl fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Server error ocurred")
    }
}