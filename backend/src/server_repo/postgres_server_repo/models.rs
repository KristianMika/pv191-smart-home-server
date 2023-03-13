use crate::server_repo::postgres_server_repo::schema::*;
use chrono::{DateTime, Local};
use diesel::{Insertable, Queryable};

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

#[derive(Insertable, Clone)]
#[diesel(table_name = measurement)]
pub struct NewMeasurementStore {
    /// Temperature in °C
    pub temperature: Option<f32>,
    /// Humidity in %
    pub humidity: Option<i32>,
    /// VOC Index in range [0,500]
    pub voc_index: Option<i32>,
    /// Time of the measurement
    pub measurement_time: Option<DateTime<Local>>,
}
