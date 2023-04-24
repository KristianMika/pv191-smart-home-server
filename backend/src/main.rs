mod auth;
mod cli;
mod display_printer;
mod endpoints;
mod middleware;
mod models;
mod request_validator;
mod sensors;
mod server_repo;
mod state;

use crate::auth::User;
use crate::display_printer::DisplayPrinter;
use crate::endpoints::{
    current_measurement::get_current_measurement, login::post_login,
    past_measurements::get_past_measurements, register::post_register,
};
use crate::{
    sensors::sampler::Sampler, server_repo::postgres_server_repo::PostgresServerRepo,
    state::ServerState,
};
use actix_cors::Cors;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{Authority, TokenSigner};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use cli::Args;
use dotenvy::dotenv;
use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use log::info;
use std::{env, io, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time};

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

    let key_pair = KeyPair::generate();

    info!("Starting server on address {}.", LISTENING_ADDRESS);
    HttpServer::new(move || {
        let authority = Authority::<User, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(key_pair.sk.clone())
                    .algorithm(Ed25519)
                    .build()
                    .expect(""),
            ))
            .verifying_key(key_pair.pk)
            .build()
            .expect("Couldn't build");
        App::new()
            .app_data(server_state.clone())
            // TODO: for development only
            .wrap(Cors::permissive())
            .service(post_register)
            .service(post_login)
            .use_jwt(
                authority,
                web::scope("/api")
                    .service(get_past_measurements)
                    .service(get_current_measurement),
            )
            .service(actix_files::Files::new("/login", WEB_FILES_PATH).index_file(INDEX_FILE))
            .service(actix_files::Files::new("/register", WEB_FILES_PATH).index_file(INDEX_FILE))
            .service(actix_files::Files::new("/", WEB_FILES_PATH).index_file(INDEX_FILE))
    })
    .bind(LISTENING_ADDRESS)?
    .run()
    .await
}
