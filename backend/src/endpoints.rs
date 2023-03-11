use actix_web::{get, HttpResponse, Responder};
use chrono::Utc;

use crate::models::{MeasurementData, MeasurementResponse};

/// Sends the currently measured values of all sensors
#[get("/api/measurement")]
pub async fn measurement() -> impl Responder {
    let temperature = 22.5;
    let humidity = 55;
    let voc_index = 120;
    let response = MeasurementResponse {
        measurement: MeasurementData {
            temperature,
            humidity,
            voc_index,
        },
        measurement_time: chrono::offset::Local::now(),
    };
    HttpResponse::Ok().json(response)
}
