mod shared;
mod workspace;
use shared::*;

use fake::{
    Fake,
    faker::lorem::en::{Paragraph, Word},
};

use almond_kernel::{
    adapters::{
        meta::RequestMeta,
        notes::{CreateNote, UpdateNote},
    },
    error::KernelError,
    repositories::prelude::NotesRepositoryExt,
};

#[tokio::test]
async fn test_create_without_workspace_notes() -> Result<(), KernelError> {
    let repo = get_notes_repository().await;

    let payload = CreateNote {
        title: Word().fake(),
        content: Paragraph(1..2).fake(),
        categories: vec![].into(),
        workspace_identifier: None,
    };

    let resp = repo.create(&payload, &None::<RequestMeta>).await;

    assert!(resp.is_err());

    Ok(())
}

#[tokio::test]
async fn test_create_with_workspace_notes() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_notes_repository).await?;

    let payload = CreateNote {
        title: Word().fake(),
        content: Paragraph(1..2).fake(),
        categories: vec![].into(),
        workspace_identifier: Some(meta.workspace_identifier),
    };

    let note = repo.create(&payload, &Some(meta.clone())).await?;

    assert_eq!(note.title, payload.title);
    assert_eq!(note.workspace_identifier, Some(meta.workspace_identifier));

    Ok(())
}

#[tokio::test]
async fn test_find_by_id_notes() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_notes_repository).await?;

    let payload = CreateNote {
        title: Word().fake(),
        content: Paragraph(1..2).fake(),
        categories: vec![].into(),
        workspace_identifier: Some(meta.workspace_identifier),
    };

    let created = repo.create(&payload, &Some(meta.clone())).await?;

    let found = repo
        .find_by_id(&created.identifier, &Some(meta.clone()))
        .await?;

    assert!(found.is_some());
    assert_eq!(found.unwrap().identifier, created.identifier);

    Ok(())
}

#[tokio::test]
async fn test_find_all_notes() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_notes_repository).await?;

    for _ in 0..3 {
        let payload = CreateNote {
            title: Word().fake(),
            content: Paragraph(1..2).fake(),
            categories: vec![].into(),
            workspace_identifier: Some(meta.workspace_identifier),
        };

        repo.create(&payload, &Some(meta.clone())).await?;
    }

    let results = repo.find_all(&Some(meta.clone())).await?;

    assert!(results.len() >= 3);

    Ok(())
}

#[tokio::test]
async fn test_recently_added_notes() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_notes_repository).await?;

    for _ in 0..5 {
        let payload = CreateNote {
            title: Word().fake(),
            content: Paragraph(1..2).fake(),
            categories: vec![].into(),
            workspace_identifier: Some(meta.workspace_identifier),
        };

        repo.create(&payload, &Some(meta.clone())).await?;
    }

    let results = repo.recently_added(&Some(meta.clone())).await?;

    assert!(results.len() <= 10);

    Ok(())
}

#[tokio::test]
async fn test_update_note() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_notes_repository).await?;

    let payload = CreateNote {
        title: Word().fake(),
        content: Paragraph(1..2).fake(),
        categories: vec![].into(),
        workspace_identifier: Some(meta.workspace_identifier),
    };

    let created = repo.create(&payload, &Some(meta.clone())).await?;

    let update = UpdateNote {
        title: Some("Updated Title".to_string()),
        content: None,
        categories: None,
    };

    let updated = repo
        .update(&created.identifier, &update, &Some(meta.clone()))
        .await?;

    assert_eq!(updated.title, "Updated Title");

    Ok(())
}

#[tokio::test]
async fn test_delete_note() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_notes_repository).await?;

    let payload = CreateNote {
        title: Word().fake(),
        content: Paragraph(1..2).fake(),
        categories: vec![].into(),
        workspace_identifier: Some(meta.workspace_identifier),
    };

    let created = repo.create(&payload, &Some(meta.clone())).await?;

    repo.delete(&created.identifier, &Some(meta.clone()))
        .await?;

    let found = repo
        .find_by_id(&created.identifier, &Some(meta.clone()))
        .await?;

    assert!(found.is_none());

    Ok(())
}
