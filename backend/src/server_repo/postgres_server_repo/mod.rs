use super::{DbError, ServerRepo};
use crate::models::{MeasurementStore, NewMeasurementStore};
use diesel::ExpressionMethods;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection, QueryDsl, RunQueryDsl,
};
use error_stack::{IntoReport, Report, ResultExt};
use std::sync::Arc;

pub(crate) mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

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
            return Err(Report::new(DbError).attach_printable(format!(
                "Invalid number of affected rows: {}",
                rows_affected
            )));
        }
        Ok(())
    }
}

impl PostgresServerRepo {
    pub fn from_url(database_url: &str) -> error_stack::Result<Self, DbError> {
        let repo = Self {
            pg_pool: Arc::new(
                PostgresServerRepo::init_pool(database_url)
                    .change_context(DbError)
                    .attach_printable_lazy(|| "Coudln't initalize pg pool")?,
            ),
        };
        // repo.apply_migrations()?;
        Ok(repo)
    }

    // pub fn apply_migrations(&self) -> error_stack::Result<(), DbError> {
    //     // let mut conn = self.get_connection()?;
    //     // // TODO: return an error instead of panicking
    //     // conn.run_pending_migrations(MIGRATIONS)
    //     //     .expect("Couldn't apply migrations");
    //     Ok(())
    // }
    fn init_pool(database_url: &str) -> error_stack::Result<PgPool, PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager).into_report()
    }

    fn get_connection(
        &self,
    ) -> error_stack::Result<PooledConnection<ConnectionManager<PgConnection>>, DbError> {
        self.pg_pool.get().into_report().change_context(DbError)
    }
}