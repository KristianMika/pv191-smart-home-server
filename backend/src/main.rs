mod endpoints;
mod http_response;
mod sensors;

use crate::sensors::sampler::Sampler;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use endpoints::measurement;
use log::info;
use std::io;

static LISTENING_ADDRESS: &str = "0.0.0.0:8080";
static WEB_FILES_PATH: &str = "/var/www/pv191-smart-home-server/";
static INDEX_FILE: &str = "index.html";

#[actix_web::main]
async fn main() -> io::Result<()> {
    // let mut sampler = Sampler::new().unwrap();
    // println!("{:?}", sampler.read_humidity_temperature().unwrap());
    env_logger::init();
    info!("Starting server on address {}.", LISTENING_ADDRESS);
    HttpServer::new(|| {
        App::new()
            // TODO: for development only
            .wrap(Cors::default().allowed_origin("http://localhost:3000"))
            .service(measurement)
            .service(actix_files::Files::new("/", WEB_FILES_PATH).index_file(INDEX_FILE))
    })
    .bind(LISTENING_ADDRESS)?
    .run()
    .await
}
