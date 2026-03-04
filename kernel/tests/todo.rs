mod shared;
mod workspace;

use almond_kernel::{
    adapters::{
        meta::RequestMeta,
        todo::{CreateTodo, TodoPriority, UpdateTodo},
    },
    error::KernelError,
    repositories::prelude::TodoRepositoryExt,
};

use chrono::Utc;
use fake::{
    Fake,
    faker::lorem::en::{Paragraph, Sentence},
};

use shared::*;

// helper to generate a CreateTodo payload
fn create_todo_payload() -> CreateTodo {
    CreateTodo {
        title: Sentence(1..3).fake(),
        description: Some(Paragraph(1..2).fake()),
        priority: TodoPriority::Medium,
        due_date: None,
    }
}

#[tokio::test]
async fn test_create_todo_without_workspace() -> Result<(), KernelError> {
    let repo = get_todo_repository().await;

    let payload = create_todo_payload();

    let resp = repo.create_todo(&payload, &None::<RequestMeta>).await;

    assert!(resp.is_err());

    Ok(())
}

#[tokio::test]
async fn test_create_todo_with_workspace() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_todo_repository).await?;

    let payload = create_todo_payload();

    let created = repo.create_todo(&payload, &Some(meta.clone())).await?;

    assert_eq!(
        created.workspace_identifier,
        Some(meta.workspace_identifier)
    );
    assert_eq!(created.done, false);

    Ok(())
}

#[tokio::test]
async fn test_find_by_id_todo() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_todo_repository).await?;

    let payload = create_todo_payload();

    let created = repo.create_todo(&payload, &Some(meta.clone())).await?;

    let found = repo
        .find_by_id(&created.identifier, &Some(meta.clone()))
        .await?;

    assert!(found.is_some());
    assert_eq!(found.unwrap().identifier, created.identifier);

    Ok(())
}

#[tokio::test]
async fn test_find_all_todos() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_todo_repository).await?;

    for _ in 0..3 {
        let payload = create_todo_payload();
        repo.create_todo(&payload, &Some(meta.clone())).await?;
    }

    let results = repo.find_all(&Some(meta.clone())).await?;

    assert!(results.len() >= 3);

    Ok(())
}

#[tokio::test]
async fn test_update_todo() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_todo_repository).await?;

    let payload = create_todo_payload();

    let created = repo.create_todo(&payload, &Some(meta.clone())).await?;

    let update = UpdateTodo {
        title: Some("Updated todo title".into()),
        description: Some("Updated description".into()),
    };

    let updated = repo
        .update(&created.identifier, &update, &Some(meta.clone()))
        .await?;

    assert_eq!(updated.title, "Updated todo title".to_string());
    assert_eq!(updated.description, Some("Updated description".into()));

    Ok(())
}

#[tokio::test]
async fn test_delete_todo() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_todo_repository).await?;

    let payload = create_todo_payload();

    let created = repo.create_todo(&payload, &Some(meta.clone())).await?;

    repo.delete(&created.identifier, &Some(meta.clone()))
        .await?;

    let found = repo
        .find_by_id(&created.identifier, &Some(meta.clone()))
        .await?;

    assert!(found.is_none());

    Ok(())
}

#[tokio::test]
async fn test_change_priority() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_todo_repository).await?;

    let payload = create_todo_payload();

    let created = repo.create_todo(&payload, &Some(meta.clone())).await?;

    let updated = repo
        .change_priority(
            &created.identifier,
            &TodoPriority::High,
            &Some(meta.clone()),
        )
        .await?;

    assert_eq!(updated.priority, "high");

    Ok(())
}

#[tokio::test]
async fn test_update_due_date() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_todo_repository).await?;

    let payload = create_todo_payload();

    let created = repo.create_todo(&payload, &Some(meta.clone())).await?;

    let due_date = Utc::now().date_naive();
    let updated = repo
        .update_due_date(&created.identifier, Some(due_date), &Some(meta.clone()))
        .await?;

    assert_eq!(updated.due_date.unwrap(), due_date);

    Ok(())
}

#[tokio::test]
async fn test_mark_done_todo() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_todo_repository).await?;

    let payload = create_todo_payload();

    let created = repo.create_todo(&payload, &Some(meta.clone())).await?;

    let updated = repo
        .mark_done(&created.identifier, true, &Some(meta.clone()))
        .await?;

    assert!(updated.done);

    Ok(())
}
