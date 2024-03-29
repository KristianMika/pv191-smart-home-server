pub mod models;
pub(crate) mod schema;

use chrono::{DateTime, Local};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::sql_types::Timestamptz;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection, QueryDsl, RunQueryDsl,
};
use diesel::{sql_query, ExpressionMethods};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use error_stack::{IntoReport, Report, ResultExt};
use std::sync::Arc;

use self::models::{MeasurementSelect, NewMeasurementStore, UserStore};
use super::{DbError, ServerRepo};
use crate::server_repo::postgres_server_repo::models::{MeasurementStore, NewUserStore};

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
            .into_report()
            .change_context(DbError::FetchError)
            .attach_printable("Couldn't fetch the last measurement")?;
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
    ) -> error_stack::Result<Vec<MeasurementSelect>, DbError> {
        let raw_query = sql_query(
            "
            SELECT interval_start as measurement_time, ROUND(AVG(temperature)::numeric, 2)::float4 as temperature, Round(AVG(humidity))::int4 as humidity, ROUND(AVG(voc_index))::int4 as voc_index
            FROM (
                SELECT date_trunc('hour', measurement_time) + (floor(date_part('minute', measurement_time) / 10) * interval '10 minute') AS interval_start,
                       temperature, humidity, voc_index
                FROM measurement
                WHERE measurement_time > $1
            ) subquery
            GROUP BY interval_start
            ORDER BY interval_start;
            "
        )
        .bind::<Timestamptz, _>(datetime_from);
        let measurements: Vec<MeasurementSelect> = raw_query
            .load::<MeasurementSelect>(&mut self.get_connection()?)
            .into_report()
            .change_context(DbError::FetchError)
            .attach_printable(format!(
                "Couldn't fetch measurements since {}",
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

    fn get_user(&self, login: &str) -> error_stack::Result<Option<UserStore>, DbError> {
        use schema::usercontext::dsl::*;

        let result: Option<UserStore> = usercontext
            .filter(user_login.eq(login))
            .limit(1)
            .load::<UserStore>(&mut self.get_connection()?)
            .into_report()
            .change_context(DbError::FetchError)
            .attach_printable(format!("Couldn't get user with login {login}"))?
            .first()
            .cloned();

        Ok(result)
    }
    fn get_user_by_id(&self, user_id: i32) -> error_stack::Result<Option<UserStore>, DbError> {
        use schema::usercontext::dsl::*;

        let result: Option<UserStore> = usercontext
            .filter(id.eq(user_id))
            .limit(1)
            .load::<UserStore>(&mut self.get_connection()?)
            .into_report()
            .change_context(DbError::FetchError)
            .attach_printable(format!("Couldn't get user with id {user_id}"))?
            .first()
            .cloned();

        Ok(result)
    }

    fn delete_measurements_older_than(
        &self,
        to: DateTime<Local>,
    ) -> error_stack::Result<usize, DbError> {
        use schema::measurement::dsl::*;

        diesel::delete(measurement.filter(measurement_time.lt(to)))
            .execute(&mut self.get_connection()?)
            .into_report()
            .change_context(DbError::FetchError)
            .attach_printable(format!("Couldn't remove measurements older than {to}"))
    }
}

impl PostgresServerRepo {
    pub fn from_url(database_url: &str) -> error_stack::Result<Self, DbError> {
        let repo = Self {
            pg_pool: Arc::new(
                PostgresServerRepo::init_pool(database_url)
                    .change_context(DbError::InitError)
                    .attach_printable_lazy(|| "Couldn't initalize pg pool")?,
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
