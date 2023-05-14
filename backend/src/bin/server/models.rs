use actix_jwt_auth_middleware::FromRequest;
use chrono::{DateTime, Local};
use common::server_repo::postgres_server_repo::models::MeasurementStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromRequest, Default)]
pub(crate) struct UserClaims {
    id: u64,
}

impl UserClaims {
    pub fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}

/// A struct used to respond to a single measurement request
#[derive(Serialize)]
pub struct MeasurementResponse {
    /// Temperature in °C
    pub temperature: Option<f32>,
    /// Humidity in %
    pub humidity: Option<u32>,
    /// VOC Index in range [0,500]
    pub voc_index: Option<u32>,
    pub measurement_time: DateTime<Local>,
}

impl From<MeasurementStore> for MeasurementResponse {
    fn from(value: MeasurementStore) -> MeasurementResponse {
        MeasurementResponse {
            temperature: value.temperature,
            humidity: value.humidity.map(|value| value as u32),
            voc_index: value.voc_index.map(|value| value as u32),
            measurement_time: value.measurement_time,
        }
    }
}
