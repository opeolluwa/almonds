use almond_kernel::kernel::Kernel;
use almond_kernel::repositories::bookmarks::BookmarkRepository;
use almond_kernel::repositories::prelude::{BookmarkRepositoryExt, WorkspaceRepositoryExt};
use almond_kernel::repositories::workspace::WorkspaceRepository;
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub fn remove_test_db() -> std::io::Result<()> {
    let path = Path::new("almond.dev.test.db");

    if path.exists() {
        fs::remove_file(path)?;
    }

    Ok(())
}

static DATABASE_PATH: &str = "sqlite://almond.dev.test.db?mode=rwc";
static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> &'static DatabaseConnection {
    DB_CONN
        .get_or_init(|| async {
            let kernel = Kernel::new(DATABASE_PATH)
                .await
                .expect("error creating kernel");

            kernel
                .run_migrations()
                .await
                .expect("error running migration");

            kernel.connection().clone()
        })
        .await
}



static WORKSPACE_REPO: OnceCell<WorkspaceRepository> = OnceCell::const_new();

pub async fn get_workspace_repository() -> &'static WorkspaceRepository {
    WORKSPACE_REPO
        .get_or_init(|| async {
            let db: DatabaseConnection = get_db().await.to_owned();
            WorkspaceRepository::new(Arc::new(db.to_owned()))
        })
        .await
}



static BOOKMARK_REPO: OnceCell<BookmarkRepository> = OnceCell::const_new();

pub async fn get_bookmark_repository() -> &'static BookmarkRepository {
    BOOKMARK_REPO
        .get_or_init(|| async {
            let db = get_db().await;
            BookmarkRepository::new(Arc::new(db.to_owned()))
        })
        .await
}