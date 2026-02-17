use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::error::KernelError;

pub struct Kernel {
    database_connection: DatabaseConnection,
}

impl Kernel {
    pub async fn new(database_url: &str) -> Result<Self, KernelError> {
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false) // disable SQLx logging
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("almond_schema"); // set default Postgres schema

        let db = Database::connect(opt)
            .await
            .map_err(|e| KernelError::DbConnectError(e.to_string()))?;

        Ok(Self {
            database_connection: db,
        })
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.database_connection
    }

    pub async fn run_migrations(&self) -> Result<(), KernelError> {
        Migrator::up(&self.database_connection, None)
            .await
            .map_err(|e| KernelError::DbConnectError(e.to_string()))?;

        Ok(())
    }
}
