mod endpoints;
mod http_response;
use log::info;
use std::io;

use actix_web::{App, HttpServer};
use endpoints::measurement;

static LISTENING_ADDRESS: &str = "0.0.0.0:8080";
static WEB_FILES_PATH: &str = "/var/www/pv191-smart-home-server/";
static INDEX_FILE: &str = "index.html";

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    info!("Starting server on address {}.", LISTENING_ADDRESS);
    HttpServer::new(|| {
        App::new()
            .service(measurement)
            .service(actix_files::Files::new("/", WEB_FILES_PATH).index_file(INDEX_FILE))
    })
    .bind(LISTENING_ADDRESS)?
    .run()
    .await
}
