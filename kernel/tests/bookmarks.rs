mod shared;
mod workspace;

use std::sync::Arc;

use almond_kernel::{
    adapters::{
        bookmarks::{BookmarkTag, CreateBookmark, UpdateBookmark},
        meta::RequestMeta,
        workspace::CreateWorkspace,
    },
    error::KernelError,
    repositories::{
        bookmarks::BookmarkRepository,
        prelude::{BookmarkRepositoryExt, WorkspaceRepositoryExt},
    },
};

use fake::{
    Fake,
    faker::{
        company::en::Industry,
        internet::en::IPv6,
        lorem::en::{Paragraph, Word},
    },
};

use shared::*;

use tokio::sync::OnceCell;



async fn setup_workspace() -> Result<(RequestMeta, BookmarkRepository), KernelError> {
    let repo = get_bookmark_repository().await.clone();
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
        repo.to_owned(),
    ))
}

#[tokio::test]
async fn test_create_without_workspace_bookmarks() -> Result<(), KernelError> {
    let repo = get_bookmark_repository().await;

    let payload = CreateBookmark {
        title: Industry().fake(),
        url: IPv6().fake(),
        tag: BookmarkTag::Design,
    };

    let resp = repo.create(&payload, None).await;

    assert!(resp.is_err());

    Ok(())
}

#[tokio::test]
async fn test_create_with_workspace_bookmarks() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace().await?;

    let payload = CreateBookmark {
        title: Industry().fake(),
        url: IPv6().fake(),
        tag: BookmarkTag::Design,
    };

    let bookmark = repo.create(&payload, Some(meta.clone())).await?;

    assert_eq!(bookmark.title, payload.title);
    assert_eq!(
        bookmark.workspace_identifier,
        Some(meta.workspace_identifier)
    );

    Ok(())
}

#[tokio::test]
async fn test_find_by_id() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace().await?;

    let payload = CreateBookmark {
        title: Industry().fake(),
        url: IPv6().fake(),
        tag: BookmarkTag::Design,
    };

    let created = repo.create(&payload, Some(meta.clone())).await?;

    let found = repo
        .find_by_id(&created.identifier, &Some(meta.clone()))
        .await?;

    assert!(found.is_some());
    assert_eq!(found.unwrap().identifier, created.identifier);

    Ok(())
}

#[tokio::test]
async fn test_find_all() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace().await?;

    for _ in 0..3 {
        let payload = CreateBookmark {
            title: Industry().fake(),
            url: IPv6().fake(),
            tag: BookmarkTag::Design,
        };

        repo.create(&payload, Some(meta.clone())).await?;
    }

    let results = repo.find_all(&Some(meta.clone())).await?;

    assert!(results.len() >= 3);

    Ok(())
}

#[tokio::test]
async fn test_find_by_tag() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace().await?;

    let payload = CreateBookmark {
        title: Industry().fake(),
        url: IPv6().fake(),
        tag: BookmarkTag::Design,
    };

    repo.create(&payload, Some(meta.clone())).await?;

    let results = repo
        .find_by_tag(&BookmarkTag::Design, &Some(meta.clone()))
        .await?;

    assert!(!results.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_recently_added() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace().await?;

    for _ in 0..5 {
        let payload = CreateBookmark {
            title: Industry().fake(),
            url: IPv6().fake(),
            tag: BookmarkTag::Design,
        };

        repo.create(&payload, Some(meta.clone())).await?;
    }

    let results = repo.recently_added(&Some(meta.clone())).await?;

    assert!(results.len() <= 10);

    Ok(())
}

#[tokio::test]
async fn test_update_bookmark() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace().await?;

    let payload = CreateBookmark {
        title: Industry().fake(),
        url: IPv6().fake(),
        tag: BookmarkTag::Design,
    };

    let created = repo.create(&payload, Some(meta.clone())).await?;

    let update = UpdateBookmark {
        title: Some("Updated Title".to_string()),
        url: None,
        tag: None,
    };

    let updated = repo
        .update(&created.identifier, &update, &Some(meta.clone()))
        .await?;

    assert_eq!(updated.title, "Updated Title");

    Ok(())
}

#[tokio::test]
async fn test_delete_bookmark() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace().await?;

    let payload = CreateBookmark {
        title: Industry().fake(),
        url: IPv6().fake(),
        tag: BookmarkTag::Design,
    };

    let created = repo.create(&payload, Some(meta.clone())).await?;

    repo.delete(&created.identifier, &Some(meta.clone()))
        .await?;

    let found = repo
        .find_by_id(&created.identifier, &Some(meta.clone()))
        .await?;

    assert!(found.is_none());

    Ok(())
}
