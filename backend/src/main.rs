mod endpoints;
mod models;
mod sensors;
mod server_repo;
mod state;

use crate::{
    sensors::sampler::Sampler, server_repo::postgres_server_repo::PostgresServerRepo,
    state::ServerState,
};
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use dotenvy::dotenv;
use endpoints::measurement;
use log::info;
use std::time::Duration;
use std::{env, io, sync::Arc};
use tokio::{task, time}; // 1.3.0

static LISTENING_ADDRESS: &str = "0.0.0.0:8080";
static WEB_FILES_PATH: &str = "/var/www/pv191-smart-home-server/";
static INDEX_FILE: &str = "index.html";
const DATABASE_URL_ENV: &str = "DATABASE_URL";
const PERIODIC_SENSOR_SAMPLING_INTERVAL_SECONDS: u64 = 30;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let server_state = ServerState {
        repo: Arc::new(
            PostgresServerRepo::from_url(
                &env::var(DATABASE_URL_ENV).expect(&format!("{} must be set", DATABASE_URL_ENV)),
            )
            .unwrap(),
        ),
        sampler: Arc::new(Sampler::new().unwrap()),
    };
    let server_state = Data::new(server_state);
    let server_state_sampling = server_state.clone();
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(
            PERIODIC_SENSOR_SAMPLING_INTERVAL_SECONDS,
        ));

        loop {
            interval.tick().await;
            match server_state_sampling.sample_sensors() {
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
            .service(measurement)
            .service(actix_files::Files::new("/", WEB_FILES_PATH).index_file(INDEX_FILE))
    })
    .bind(LISTENING_ADDRESS)?
    .run()
    .await
}
