use axum::Router;
use axum::routing::get;

use axum_test::TestServer;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::test]
async fn test_health_check() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://test.db")
        .await
        .unwrap();
    let app = uralium_lib::routes::router::load_routes(&pool);
}
