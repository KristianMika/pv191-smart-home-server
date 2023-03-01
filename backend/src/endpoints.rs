use actix_web::{get, HttpResponse, Responder};

use crate::http_response::MeasurementResponse;

/// Sends the currently measured values of all sensors
#[get("/api/measurement")]
pub async fn measurement() -> impl Responder {
    let temperature = 22.5;
    let humidity = 55;
    let pressure = 1020;
    let response = MeasurementResponse {
        temperature,
        humidity,
        pressure,
    };
    HttpResponse::Ok().json(response)
}
