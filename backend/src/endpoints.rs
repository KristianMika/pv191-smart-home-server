use actix_web::{get, HttpResponse, Responder};

use crate::models::{MeasurementData, MeasurementResponse};

/// Sends the currently measured values of all sensors
#[get("/api/measurement")]
pub async fn measurement() -> impl Responder {
    let temperature = Some(22.5);
    let humidity = Some(55);
    let voc_index = Some(120);
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
