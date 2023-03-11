use crate::server_repo::postgres_server_repo::schema::*;
use chrono::{DateTime, Local, Utc};
use diesel::{Insertable, Queryable};
use serde::Serialize;

/// A struct used to respond to a single measurement request
#[derive(Serialize)]
pub struct MeasurementResponse {
    pub measurement: MeasurementData,
    pub measurement_time: DateTime<Local>,
}

#[derive(Serialize)]
pub struct MeasurementData {
    /// Temperature in °C
    pub temperature: f32,
    /// Humidity in %
    pub humidity: u32,
    /// VOC Index in range [0,500]
    pub voc_index: u32,
}

#[derive(Debug, Queryable, Clone)]
#[diesel(table_name = measurement)]
pub struct MeasurementStore {
    pub id: i32,
    /// Temperature in °C
    pub temperature: Option<f32>,
    /// Humidity in %
    pub humidity: Option<i32>,
    /// VOC Index in range [0,500]
    pub voc_index: Option<i32>,
    /// Time of the measurement
    pub measurement_time: DateTime<Local>,
}

#[derive(Insertable)]
#[diesel(table_name = measurement)]
pub struct NewMeasurementStore {
    /// Temperature in °C
    pub temperature: Option<f32>,
    /// Humidity in %
    pub humidity: Option<i32>,
    /// VOC Index in range [0,500]
    pub voc_index: Option<i32>,
    /// Time of the measurement
    pub measurement_time: Option<DateTime<Utc>>,
}
