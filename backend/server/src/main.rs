mod cli;
mod endpoints;
mod models;
mod request_validator;
mod state;

use actix_cors::Cors;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{Authority, TokenSigner};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use chrono::Duration;
use clap::Parser;
use cli::Args;
use common::server_repo::postgres_server_repo::PostgresServerRepo;
use dotenvy::dotenv;
use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use log::info;
use std::{env, io, sync::Arc};

use crate::endpoints::auth::{
    ACCESS_TOKEN_COOKIE_NAME, ACCESS_TOKEN_MAX_AGE_MINUTES, REFRESH_TOKEN_COOKIE_NAME,
    REFRESH_TOKEN_MAX_AGE_DAYS,
};
use crate::endpoints::current_measurement::get_current_measurement;
use crate::endpoints::login::post_login;
use crate::endpoints::logout::post_logout;
use crate::endpoints::past_measurements::get_past_measurements;
use crate::endpoints::register::post_register;
use crate::endpoints::user::get_user;
use crate::models::UserClaims;
use crate::state::ServerState;

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
    };

    let server_state = Data::new(server_state);

    let key_pair = KeyPair::generate();

    let args_cloned = args.clone();
    let listening_address = args_cloned.get_listening_address();
    info!("Starting server on address {}.", listening_address);
    HttpServer::new(move || {
        let authority = Authority::<UserClaims, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .access_token_name(ACCESS_TOKEN_COOKIE_NAME)
                    .access_token_lifetime(Duration::minutes(ACCESS_TOKEN_MAX_AGE_MINUTES))
                    .refresh_token_name(REFRESH_TOKEN_COOKIE_NAME)
                    .refresh_token_lifetime(Duration::days(REFRESH_TOKEN_MAX_AGE_DAYS))
                    .signing_key(key_pair.sk.clone())
                    .algorithm(Ed25519)
                    .build()
                    .expect("Couldn't build TokenSigner"),
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
            .service(post_logout)
            .use_jwt(
                authority,
                web::scope("/api")
                    .service(get_past_measurements)
                    .service(get_current_measurement)
                    .service(get_user),
            )
            .service(
                actix_files::Files::new("/login", args.get_web_files_path())
                    .index_file(args.get_index_filename()),
            )
            .service(
                actix_files::Files::new("/logout", args.get_web_files_path())
                    .index_file(args.get_index_filename()),
            )
            .service(
                actix_files::Files::new("/register", args.get_web_files_path())
                    .index_file(args.get_index_filename()),
            )
            .service(
                actix_files::Files::new("/forbidden", args.get_web_files_path())
                    .index_file(args.get_index_filename()),
            )
            .service(
                actix_files::Files::new("/", args.get_web_files_path())
                    .index_file(args.get_index_filename()),
            )
    })
    .bind(listening_address)?
    .run()
    .await
}
