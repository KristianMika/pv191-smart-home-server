mod endpoints;
mod http_response;
use std::io;

use actix_web::{App, HttpServer};
use endpoints::measurement;

static LISTENING_ADDRESS: &str = "0.0.0.0:8080";

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(measurement))
        .bind(LISTENING_ADDRESS)?
        .run()
        .await
}
