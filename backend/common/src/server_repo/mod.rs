use self::postgres_server_repo::models::{MeasurementStore, NewMeasurementStore, UserStore};
use chrono::{DateTime, Local};
use std::{error::Error, fmt};

pub mod postgres_server_repo;

/// Repository trait for database backends
pub trait ServerRepo {
    /// Stores the latest measurement
    fn store_measurement(
        &self,
        measurement: NewMeasurementStore,
    ) -> error_stack::Result<(), DbError>;

    /// Fatches the latest measurement
    fn get_last_measurement(&self) -> error_stack::Result<Option<MeasurementStore>, DbError>;

    /// Fetches all measurements that were measured after `from`
    fn get_measurements_from(
        &self,
        from: DateTime<Local>,
    ) -> error_stack::Result<Vec<MeasurementStore>, DbError>;

    fn create_new_user(
        &self,
        first_name: &str,
        user_login: &str,
        user_password_hash: &str,
    ) -> error_stack::Result<UserStore, DbError>;

    /// Fetches a user record of the user with the specific `login`
    fn get_user(&self, login: &str) -> error_stack::Result<Option<UserStore>, DbError>;

    fn get_user_by_id(&self, id: i32) -> error_stack::Result<Option<UserStore>, DbError>;
}

#[derive(Debug)]
pub enum DbError {
    ConstraintError,
    InitError,
    GeneralError,
    StoreError,
    FetchError,
}

impl Error for DbError {}
impl fmt::Display for DbError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("could not interract with the db")
    }
}
