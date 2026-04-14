use std::env;

use almond_kernel::{error::KernelError, utils::extract_env};
use dotenv::dotenv;
use tower_http::cors::AllowOrigin;

pub struct AppConfig {
    pub database_url: String,
    pub max_db_connections: u32,
    pub body_limit_mb: usize,
    pub upload_path: String,
    pub export_path: String,
    pub port: u16,
    pub environment: String,
    pub allowed_origins: AllowOrigin,

    // GraphQL / API settings
    pub graphql_endpoint: String,
    pub depth_limit: Option<usize>,
    pub complexity_limit: Option<usize>,

    pub base_url: String,
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppConfig")
            .field("database_url", &"****")
            .field("max_db_connections", &self.max_db_connections)
            .field("body_limit_mb", &self.body_limit_mb)
            .field("upload_path", &self.upload_path)
            .field("export_path", &self.export_path)
            .field("port", &self.port)
            .field("environment", &self.environment)
            .field("allowed_origins", &self.allowed_origins)
            .field("graphql_endpoint", &self.graphql_endpoint)
            .field("depth_limit", &self.depth_limit)
            .field("complexity_limit", &self.complexity_limit)
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, KernelError> {
        dotenv().ok();

        let port = extract_env::<u16>("PORT")?;
        let max_db_connections = extract_env::<u32>("MAX_DB_CONNECTIONS")?;
        let body_limit_mb = extract_env::<usize>("BODY_LIMIT_MB")?;

        let export_path = extract_env("EXPORT_PATH").unwrap_or_else(|_| "/tmp".to_string());
        let upload_path = extract_env("UPLOAD_PATH").unwrap_or_else(|_| "/tmp".to_string());

        let environment = extract_env("ENVIRONMENT")?;

        let graphql_endpoint = env::var("GRAPHQL_ENDPOINT").unwrap_or_else(|_| "/orchard".into());
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| format!("http://localhost:{port}"));

        let depth_limit = env::var("DEPTH_LIMIT")
            .ok()
            .map(|v| v.parse().expect("DEPTH_LIMIT is not a number"));

        let complexity_limit = env::var("COMPLEXITY_LIMIT")
            .ok()
            .map(|v| v.parse().expect("COMPLEXITY_LIMIT is not a number"));

        // Parse allowed origins (comma-separated)
        let allowed_origins = match extract_env::<String>("ALLOWED_ORIGINS").as_deref() {
            Ok("*") | Err(_) => AllowOrigin::any(),
            Ok(origins) => {
                let parsed = origins
                    .split(',')
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
            graphql_endpoint,
            depth_limit,
            complexity_limit,
            base_url,
        })
    }
}
