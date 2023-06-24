use actix_web::{get, web, HttpResponse, Responder};
use common::server_repo::ServerRepo;
use log::error;

use crate::{models::MeasurementResponse, state::ServerState};

/// Sends the currently measured values of all sensors
#[get("/measurement")]
pub async fn get_current_measurement(state: web::Data<ServerState>) -> impl Responder {
    let response: MeasurementResponse = match state.get_repo().get_last_measurement() {
        Err(err) => {
            error!("{:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
        Ok(measurement) => measurement.unwrap_or_default().into(),
    };

    HttpResponse::Ok().json(response)
}
