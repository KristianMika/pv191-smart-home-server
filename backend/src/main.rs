mod endpoints;
mod models;
mod sensors;
mod server_repo;
mod state;

use crate::{
    models::NewMeasurementStore,
    sensors::sampler::Sampler,
    server_repo::{postgres_server_repo::PostgresServerRepo, ServerRepo},
    state::ServerState,
};
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use dotenvy::dotenv;
use endpoints::measurement;
use log::info;
use std::{env, io, sync::Arc};

static LISTENING_ADDRESS: &str = "0.0.0.0:8080";
static WEB_FILES_PATH: &str = "/var/www/pv191-smart-home-server/";
static INDEX_FILE: &str = "index.html";
const DATABASE_URL_ENV: &str = "DATABASE_URL";

#[actix_web::main]
async fn main() -> io::Result<()> {
    // let mut sampler = Sampler::new().unwrap();
    // println!("{:?}", sampler.read_humidity_temperature().unwrap());
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
