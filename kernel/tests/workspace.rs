// mod shared;

// use std::sync::Arc;
// use almond_kernel::repositories::{
//     prelude::WorkspaceRepositoryExt, workspace::WorkspaceRepository,
// };

// use sea_orm::DatabaseConnection;
// use tokio::sync::OnceCell;

// static WORKSPACE_REPO: OnceCell<WorkspaceRepository> = OnceCell::const_new();

// pub async fn get_workspace_repository() -> &'static WorkspaceRepository {
//     WORKSPACE_REPO
//         .get_or_init(|| async {
//             let db: DatabaseConnection = get_db().await.to_owned();
//             WorkspaceRepository::new(Arc::new(db.to_owned()))
//         })
//         .await
// }
