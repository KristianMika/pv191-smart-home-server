use self::models::{NewMeasurementStore, UserStore};
use super::{DbError, ServerRepo};
use crate::server_repo::postgres_server_repo::models::{MeasurementStore, NewUserStore};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::ExpressionMethods;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection, QueryDsl, RunQueryDsl,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use error_stack::{IntoReport, Report, ResultExt};
use std::sync::Arc;

pub(crate) mod models;
pub(crate) mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Implementation of `ServerRepo` for PostgreSQL DB
pub struct PostgresServerRepo {
    /// Postgres connection pool
    pg_pool: Arc<PgPool>,
}

impl ServerRepo for PostgresServerRepo {
    fn get_last_measurement(&self) -> error_stack::Result<Option<MeasurementStore>, DbError> {
        use schema::measurement::dsl::*;
        let result = measurement
            .order(measurement_time.desc())
            .limit(1)
            .load::<MeasurementStore>(&mut self.get_connection()?)
            .unwrap();
        Ok(result.first().cloned())
    }

    fn store_measurement(
        &self,
        measurement: NewMeasurementStore,
    ) -> error_stack::Result<(), DbError> {
        use schema::measurement;
        let expected_rows_affected = 1;
        let rows_affected: usize = diesel::insert_into(measurement::table)
            .values(&measurement)
            .execute(&mut self.get_connection()?)
            .expect("Error saving new post");
        if rows_affected != expected_rows_affected {
            return Err(Report::new(DbError::StoreError).attach_printable(format!(
                "Invalid number of affected rows: {}",
                rows_affected
            )));
        }
        Ok(())
    }

    fn get_measurements_from(
        &self,
        datetime_from: chrono::DateTime<chrono::Local>,
    ) -> error_stack::Result<Vec<MeasurementStore>, DbError> {
        use schema::measurement::dsl::*;

        let measurements: Vec<MeasurementStore> = measurement
            .filter(measurement_time.ge(datetime_from))
            .load::<MeasurementStore>(&mut self.get_connection()?)
            .into_report()
            .change_context(DbError::FetchError)
            .attach_printable(format!(
                "Couldn't fetch measurements from {}",
                datetime_from
            ))?;
        Ok(measurements)
    }

    fn create_new_user(
        &self,
        first_name: &str,
        user_login: &str,
        user_password_hash: &str,
    ) -> error_stack::Result<UserStore, DbError> {
        use schema::usercontext;

        let new_user = NewUserStore {
            first_name,
            user_login,
            user_password_hash,
        };
        let created_user = diesel::insert_into(usercontext::table)
            .values(&new_user)
            .get_result(&mut self.get_connection()?)
            .map_err(|err| match err {
                DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    Report::new(DbError::ConstraintError)
                }
                other_err => Report::new(DbError::StoreError)
                    .attach_printable(format!("Couldn't store user: {other_err}")),
            })?;

        Ok(created_user)
    }
}

impl PostgresServerRepo {
    pub fn from_url(database_url: &str) -> error_stack::Result<Self, DbError> {
        let repo = Self {
            pg_pool: Arc::new(
                PostgresServerRepo::init_pool(database_url)
                    .change_context(DbError::InitError)
                    .attach_printable_lazy(|| "Coudln't initalize pg pool")?,
            ),
        };
        repo.apply_migrations()?;
        Ok(repo)
    }

    pub fn apply_migrations(&self) -> error_stack::Result<(), DbError> {
        let mut conn = self.get_connection()?;
        if let Err(err) = conn.run_pending_migrations(MIGRATIONS) {
            return Err(Report::new(DbError::InitError)
                .attach_printable(format!("Couldn't apply migrations: {}", err)));
        }

        Ok(())
    }
    fn init_pool(database_url: &str) -> error_stack::Result<PgPool, PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager).into_report()
    }

    fn get_connection(
        &self,
    ) -> error_stack::Result<PooledConnection<ConnectionManager<PgConnection>>, DbError> {
        self.pg_pool
            .get()
            .into_report()
            .change_context(DbError::GeneralError)
    }
}
