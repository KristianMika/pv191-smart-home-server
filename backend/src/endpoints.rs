use actix_web::{get, web, HttpResponse, Responder};
use log::error;

use crate::{models::MeasurementResponse, server_repo::ServerRepo, state::ServerState};

/// Sends the currently measured values of all sensors
#[get("/api/measurement")]
pub async fn measurement(state: web::Data<ServerState>) -> impl Responder {
    let response: MeasurementResponse = match state.repo.get_last_measurement() {
        Err(err) => {
            error!("{:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
        Ok(measurement) => measurement.unwrap_or_default().into(),
    };

    HttpResponse::Ok().json(response)
}
