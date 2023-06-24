use chrono::{DateTime, Local};
use diesel::sql_types::{Float4, Int4, Nullable, Timestamptz};
use diesel::{Insertable, Queryable, QueryableByName};
use serde::Serialize;

use crate::server_repo::postgres_server_repo::schema::*;

#[derive(Debug, Queryable, Clone, Default, QueryableByName)]
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

#[derive(Debug, Queryable, Clone, Default, QueryableByName, Serialize)]
pub struct MeasurementSelect {
    /// Temperature in °C
    #[diesel(sql_type = Nullable<Float4>)]
    pub temperature: Option<f32>,
    /// Humidity in %
    #[diesel(sql_type = Nullable<Int4>)]
    pub humidity: Option<i32>,
    /// VOC Index in range [0,500]
    #[diesel(sql_type = Nullable<Int4>)]
    pub voc_index: Option<i32>,
    /// Time of the measurement
    #[diesel(sql_type = Timestamptz)]
    pub measurement_time: DateTime<Local>,
}

#[derive(Insertable, Clone, Copy)]
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

#[derive(Debug, Queryable, Clone, Default)]
#[diesel(table_name = usercontext)]
pub struct UserStore {
    pub id: i32,
    pub first_name: String,
    pub user_login: String,
    pub user_password_hash: String,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = usercontext)]
pub struct NewUserStore<'a> {
    pub first_name: &'a str,
    pub user_login: &'a str,
    pub user_password_hash: &'a str,
}
