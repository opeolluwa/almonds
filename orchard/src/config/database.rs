use std::{sync::Arc, time::Duration};

use migration::{Migrator, MigratorTrait};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{config::AppConfig, errors::app_error::AppError};

pub struct AppDatabase {
    pub db_conn: Arc<DatabaseConnection>,
}

impl AppDatabase {
    pub async fn init(config: &AppConfig) -> Result<Self, AppError> {
        let mut opt = ConnectOptions::new(&config.database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db_conn = Database::connect(opt).await?;

        Migrator::up(&db_conn, None).await?;

        tracing::info!("Database migrations completed");

        Ok(Self {
            db_conn: Arc::new(db_conn),
        })
    }

    pub fn conn(&self) -> Arc<DatabaseConnection> {
        self.db_conn.clone()
    }
}
