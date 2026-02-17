use std::sync::Arc;

use almond_kernel::sea_orm::DatabaseConnection;

pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
}
