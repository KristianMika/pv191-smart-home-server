use actix_web::{get, web, HttpResponse, Responder};
use chrono::{Duration, Local};
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

/// Sends history measurements within the last 24 hours
#[get("/api/past_measurements")]
pub async fn past_measurements(state: web::Data<ServerState>) -> impl Responder {
    let one_day_ago = Local::now() - Duration::days(1);

    let response: Vec<MeasurementResponse> = match state.repo.get_measurements_from(one_day_ago) {
        Err(err) => {
            error!("{:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
        Ok(measurements) => measurements
            .into_iter()
            .map(|measurement_store| measurement_store.into())
            .collect(),
    };

    HttpResponse::Ok().json(response)
}
