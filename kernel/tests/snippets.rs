#![cfg(feature = "sqlite")]
mod shared;
mod workspace;
use shared::*;

use chrono::Local;
use fake::{
    Fake,
    faker::lorem::en::{Paragraph, Sentence, Word},
};

use almond_kernel::{
    adapters::{
        meta::RequestMeta,
        snippets::{CreateSnippet, UpdateSnippet},
    },
    error::KernelError,
    repositories::prelude::SnippetRepositoryExt,
};

#[tokio::test]
async fn test_create_without_workspace_snippets() -> Result<(), KernelError> {
    let repo = get_snippets_repository().await;

    let payload = CreateSnippet {
        title: Some(Sentence(1..2).fake()),
        language: Some(Word().fake()),
        code: Paragraph(1..2).fake(),
        description: Some(Paragraph(1..2).fake()),
        is_pinned: false,
        workspace_identifier: None,
        created_at: Local::now().into(),
        updated_at: Local::now().into(),
    };

    let resp = repo.create(&payload, &None::<RequestMeta>).await;

    assert!(resp.is_err());

    Ok(())
}

#[tokio::test]
async fn test_create_with_workspace_snippets() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_snippets_repository).await?;

    let payload = CreateSnippet {
        title: Some(Sentence(1..2).fake()),
        language: Some(Word().fake()),
        code: Paragraph(1..2).fake(),
        description: Some(Paragraph(1..2).fake()),
        is_pinned: false,
        workspace_identifier: Some(meta.workspace_identifier),
        created_at: Local::now().into(),
        updated_at: Local::now().into(),
    };

    let snippet = repo.create(&payload, &Some(meta.clone())).await?;

    assert_eq!(
        snippet.workspace_identifier,
        Some(meta.workspace_identifier)
    );

    Ok(())
}

#[tokio::test]
async fn test_find_by_id_snippets() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_snippets_repository).await?;

    let payload = CreateSnippet {
        title: Some(Sentence(1..2).fake()),
        language: Some(Word().fake()),
        code: Paragraph(1..2).fake(),
        description: Some(Paragraph(1..2).fake()),
        is_pinned: false,
        workspace_identifier: Some(meta.workspace_identifier),
        created_at: Local::now().into(),
        updated_at: Local::now().into(),
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
async fn test_find_all_snippets() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_snippets_repository).await?;

    for _ in 0..3 {
        let payload = CreateSnippet {
            title: Some(Sentence(1..2).fake()),
            language: Some(Word().fake()),
            code: Paragraph(1..2).fake(),
            description: Some(Paragraph(1..2).fake()),
            is_pinned: false,
            workspace_identifier: Some(meta.workspace_identifier),
            created_at: Local::now().into(),
            updated_at: Local::now().into(),
        };

        repo.create(&payload, &Some(meta.clone())).await?;
    }

    let results = repo.find_all(&Some(meta.clone())).await?;

    assert!(results.len() >= 3);

    Ok(())
}

#[tokio::test]
async fn test_recently_added_snippets() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_snippets_repository).await?;

    for _ in 0..5 {
        let payload = CreateSnippet {
            title: Some(Sentence(1..2).fake()),
            language: Some(Word().fake()),
            code: Paragraph(1..2).fake(),
            description: Some(Paragraph(1..2).fake()),
            is_pinned: false,
            workspace_identifier: Some(meta.workspace_identifier),
            created_at: Local::now().into(),
            updated_at: Local::now().into(),
        };

        repo.create(&payload, &Some(meta.clone())).await?;
    }

    let results = repo.recently_added(&Some(meta.clone())).await?;

    assert!(results.len() <= 10);

    Ok(())
}

#[tokio::test]
async fn test_update_snippet() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_snippets_repository).await?;

    let payload = CreateSnippet {
        title: Some(Sentence(1..2).fake()),
        language: Some(Word().fake()),
        code: Paragraph(1..2).fake(),
        description: Some(Paragraph(1..2).fake()),
        is_pinned: false,
        workspace_identifier: Some(meta.workspace_identifier),
        created_at: Local::now().into(),
        updated_at: Local::now().into(),
    };

    let created = repo.create(&payload, &Some(meta.clone())).await?;

    let update = UpdateSnippet {
        title: Some("Updated Snippet".to_string()),
        language: None,
        code: None,
        description: None,
        is_pinned: Some(true),
    };

    let updated = repo
        .update(&created.identifier, &update, &Some(meta.clone()))
        .await?;

    assert_eq!(updated.title, Some("Updated Snippet".to_string()));
    assert!(updated.is_pinned);

    Ok(())
}

#[tokio::test]
async fn test_delete_snippet() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_snippets_repository).await?;

    let payload = CreateSnippet {
        title: Some(Sentence(1..2).fake()),
        language: Some(Word().fake()),
        code: Paragraph(1..2).fake(),
        description: Some(Paragraph(1..2).fake()),
        is_pinned: false,
        workspace_identifier: Some(meta.workspace_identifier),
        created_at: Local::now().into(),
        updated_at: Local::now().into(),
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
