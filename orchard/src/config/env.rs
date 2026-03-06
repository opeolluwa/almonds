use dotenv::dotenv;
use tower_http::cors::AllowOrigin;

use aers_utils::extract_env;

use crate::errors::app_error::AppError;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub max_db_connections: u32,
    pub body_limit_mb: usize,
    pub upload_path: String,
    pub export_path: String,
    pub port: u16,
    pub environment: String,
    pub allowed_origins: AllowOrigin,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        dotenv().ok();

        let port = extract_env::<u16>("PORT")?;

        let max_db_connections = extract_env::<u32>("MAX_DB_CONNECTIONS")?;

        let body_limit_mb = extract_env::<usize>("BODY_LIMIT_MB")?;

        let export_path = extract_env("EXPORT_PATH").unwrap_or_else(|_| "/tmp".to_string());
        let upload_path = extract_env("UPLOAD_PATH").unwrap_or_else(|_| "/tmp".to_string());

        let environment = extract_env("ENVIRONMENT")?;

        // Parse allowed origins (comma-separated list)
        let allowed_origins = match extract_env::<String>("ALLOWED_ORIGINS").as_deref() {
            Ok("*") | Err(_) => AllowOrigin::any(),
            Ok(origins) => {
                let parsed = origins
                    .split(",")
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| s.parse().ok())
                    .collect::<Vec<_>>();

                AllowOrigin::list(parsed)
            }
        };

        Ok(Self {
            database_url: extract_env("DATABASE_URL")?,
            max_db_connections,
            body_limit_mb,
            upload_path,
            export_path,
            port,
            environment,
            allowed_origins,
        })
    }
}
