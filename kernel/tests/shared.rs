use almond_kernel::adapters::meta::RequestMeta;
use almond_kernel::adapters::workspace::CreateWorkspace;
use almond_kernel::error::KernelError;
use almond_kernel::kernel::Kernel;
use almond_kernel::repositories::bookmarks::BookmarkRepository;
use almond_kernel::repositories::notes::{NotesRepository, NotesRepositoryExt};
use almond_kernel::repositories::prelude::{
    BookmarkRepositoryExt, RecycleBinRepositoryExt, ReminderRepositoryExt, SnippetRepositoryExt,
    WorkspaceRepositoryExt,
};
use almond_kernel::repositories::recycle_bin::RecycleBinRepository;
use almond_kernel::repositories::reminder::ReminderRepository;
use almond_kernel::repositories::snippets::SnippetRepository;
use almond_kernel::repositories::todo::{TodoRepository, TodoRepositoryExt};
use almond_kernel::repositories::workspace::WorkspaceRepository;
use fake::Fake;
use fake::faker::lorem::en::{Paragraph, Word};
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::OnceCell;
use std::future::Future;

pub fn _remove_test_db() -> std::io::Result<()> {
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

// Todo
static TODO_REPO: OnceCell<TodoRepository> = OnceCell::const_new();

pub async fn get_todo_repository() -> &'static TodoRepository {
    TODO_REPO
        .get_or_init(|| async {
            let db = get_db().await;
            TodoRepository::new(Arc::new(db.to_owned()))
        })
        .await
}

// Reminder
static REMINDER_REPO: OnceCell<ReminderRepository> = OnceCell::const_new();

pub async fn get_reminder_repository() -> &'static ReminderRepository {
    REMINDER_REPO
        .get_or_init(|| async {
            let db = get_db().await;
            ReminderRepository::new(Arc::new(db.to_owned()))
        })
        .await
}

// Recycle Bin
static RECYCLE_BIN_REPO: OnceCell<RecycleBinRepository> = OnceCell::const_new();

pub async fn get_recycle_bin_repository() -> &'static RecycleBinRepository {
    RECYCLE_BIN_REPO
        .get_or_init(|| async {
            let db = get_db().await;
            RecycleBinRepository::new(Arc::new(db.to_owned()))
        })
        .await
}

// Notes
static NOTES_REPO: OnceCell<NotesRepository> = OnceCell::const_new();

pub async fn get_notes_repository() -> &'static NotesRepository {
    NOTES_REPO
        .get_or_init(|| async {
            let db = get_db().await;
            NotesRepository::new(Arc::new(db.to_owned()))
        })
        .await
}

// Notification
// static NOTIFICATION_REPO: OnceCell<NotificationsRepository> = OnceCell::const_new();

// pub async fn get_notification_repository() -> &'static NotificationsRepository {
//     NOTIFICATION_REPO
//         .get_or_init(|| async {
//             let db = get_db().await;
//             NotificationsRepository::new(Arc::new(db.to_owned()))
//         })
//         .await
// }

// Snippets
static SNIPPETS_REPO: OnceCell<SnippetRepository> = OnceCell::const_new();

pub async fn get_snippets_repository() -> &'static SnippetRepository {
    SNIPPETS_REPO
        .get_or_init(|| async {
            let db = get_db().await;
            SnippetRepository::new(Arc::new(db.to_owned()))
        })
        .await
}




pub async fn setup_workspace<R, F, Fut>(repo_getter: F) -> Result<(RequestMeta, R), KernelError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = &'static R> + 'static,
    R: Clone + 'static,
{
    let repo = repo_getter().await.clone();
    let workspace_repo = get_workspace_repository().await;

    let workspace = workspace_repo
        .create_workspace(CreateWorkspace {
            name: Word().fake(),
            description: Paragraph(1..2).fake(),
        })
        .await?;

    Ok((
        RequestMeta {
            workspace_identifier: workspace.identifier,
        },
        repo,
    ))
}