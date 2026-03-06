use sea_orm::DatabaseConnection;

use crate::events::subscriber::{EventSubscriber, EventSubscriberExt};

pub struct AppBackgroundTasks {}

impl AppBackgroundTasks {
    pub fn init(db: std::sync::Arc<DatabaseConnection>) {
        tokio::spawn(async move {
            if let Err(err) = EventSubscriber::start_redis_listener(&db).await {
                tracing::error!("Redis listener failed: {err}");
            }
        });
    }
}
