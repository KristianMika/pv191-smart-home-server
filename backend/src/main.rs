mod cli;
mod display_printer;
mod endpoints;
mod models;
mod sensors;
mod server_repo;
mod state;

use crate::display_printer::DisplayPrinter;
use crate::endpoints::current_measurement::get_current_measurement;
use crate::endpoints::past_measurements::get_past_measurements;
use crate::{
    sensors::sampler::Sampler, server_repo::postgres_server_repo::PostgresServerRepo,
    state::ServerState,
};
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use cli::Args;
use dotenvy::dotenv;
use log::info;
use std::time::Duration;
use std::{env, io, sync::Arc};
use tokio::sync::Mutex;
use tokio::{task, time}; // 1.3.0

static LISTENING_ADDRESS: &str = "0.0.0.0:8080";
static WEB_FILES_PATH: &str = "/var/www/pv191-smart-home-server/";
static INDEX_FILE: &str = "index.html";
const DATABASE_URL_ENV: &str = "DATABASE_URL";

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let args = Args::parse();

    let db_url =
        env::var(DATABASE_URL_ENV).unwrap_or_else(|_| panic!("{} must be set", DATABASE_URL_ENV));
    let server_state = ServerState {
        repo: Arc::new(PostgresServerRepo::from_url(&db_url).unwrap()),
        sampler: Arc::new(Mutex::new(
            Sampler::new(args.get_dht11_pin(), args.get_voc_i2c_dev()).unwrap(),
        )),
        display_printer: Arc::new(Mutex::new(
            DisplayPrinter::new(args.get_display_i2c_dev()).unwrap(),
        )),
    };
    let server_state = Data::new(server_state);
    let server_state_sampling = server_state.clone();
    task::spawn(async move {
        let mut interval =
            time::interval(Duration::from_secs(args.get_periodic_sampling_seconds()));

        loop {
            interval.tick().await;
            match server_state_sampling.sample_sensors().await {
                Ok(_) => {}
                Err(err) => {
                    log::error!("{}", err)
                }
            };
        }
    });

    info!("Starting server on address {}.", LISTENING_ADDRESS);
    HttpServer::new(move || {
        App::new()
            .app_data(server_state.clone())
            // TODO: for development only
            .wrap(Cors::default().allowed_origin("http://localhost:3000"))
            .service(get_current_measurement)
            .service(get_past_measurements)
            .service(actix_files::Files::new("/", WEB_FILES_PATH).index_file(INDEX_FILE))
    })
    .bind(LISTENING_ADDRESS)?
    .run()
    .await
}
