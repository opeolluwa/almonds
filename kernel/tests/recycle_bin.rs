mod shared;
mod workspace;

use almond_kernel::{
    adapters::{
        meta::RequestMeta,
        recycle_bin::{CreateRecycleBinEntry, RecycleBinItemType},
    },
    error::KernelError,
    repositories::prelude::RecycleBinRepositoryExt,
};

use fake::{
    Fake,
    faker::lorem::en::Paragraph,
};

use uuid::Uuid;

use shared::*;

#[tokio::test]
async fn test_store_without_workspace_recycle_bin() -> Result<(), KernelError> {
    let repo = get_recycle_bin_repository().await;

    let payload = CreateRecycleBinEntry {
        item_id: Uuid::new_v4(),
        item_type: RecycleBinItemType::Note,
        payload: Paragraph(1..2).fake(),
        workspace_identifier: None,
    };

    let resp = repo.store(&payload, &None::<RequestMeta>).await;

    assert!(resp.is_err());

    Ok(())
}

#[tokio::test]
async fn test_store_with_workspace_recycle_bin() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_recycle_bin_repository).await?;

    let payload = CreateRecycleBinEntry {
        item_id: Uuid::new_v4(),
        item_type: RecycleBinItemType::Note,
        payload: Paragraph(1..2).fake(),
        workspace_identifier: Some(meta.workspace_identifier),
    };

    let entry = repo.store(&payload, &Some(meta.clone())).await?;

    assert_eq!(entry.item_id, payload.item_id);
    assert_eq!(entry.workspace_identifier, Some(meta.workspace_identifier));

    Ok(())
}

#[tokio::test]
async fn test_find_by_id_recycle_bin() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_recycle_bin_repository).await?;

    let payload = CreateRecycleBinEntry {
        item_id: Uuid::new_v4(),
        item_type: RecycleBinItemType::Note,
        payload: Paragraph(1..2).fake(),
        workspace_identifier: Some(meta.workspace_identifier),
    };

    let created = repo.store(&payload, &Some(meta.clone())).await?;

    let found = repo
        .find_by_id(&created.identifier, &Some(meta.clone()))
        .await?;

    assert!(found.is_some());
    assert_eq!(found.unwrap().identifier, created.identifier);

    Ok(())
}

#[tokio::test]
async fn test_find_all_recycle_bin() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_recycle_bin_repository).await?;

    for _ in 0..3 {
        let payload = CreateRecycleBinEntry {
            item_id: Uuid::new_v4(),
            item_type: RecycleBinItemType::Note,
            payload: Paragraph(1..2).fake(),
            workspace_identifier: Some(meta.workspace_identifier),
        };

        repo.store(&payload, &Some(meta.clone())).await?;
    }

    let results = repo.find_all(&Some(meta.clone())).await?;

    assert!(results.len() >= 3);

    Ok(())
}

#[tokio::test]
async fn test_find_by_item_type_recycle_bin() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_recycle_bin_repository).await?;

    for _ in 0..3 {
        let payload = CreateRecycleBinEntry {
            item_id: Uuid::new_v4(),
            item_type: RecycleBinItemType::Note,
            payload: Paragraph(1..2).fake(),
            workspace_identifier: Some(meta.workspace_identifier),
        };

        repo.store(&payload, &Some(meta.clone())).await?;
    }

    let results = repo
        .find_by_item_type(&RecycleBinItemType::Note, &Some(meta.clone()))
        .await?;

    assert!(results.len() >= 3);

    Ok(())
}

#[tokio::test]
async fn test_purge_recycle_bin() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_recycle_bin_repository).await?;

    let payload = CreateRecycleBinEntry {
        item_id: Uuid::new_v4(),
        item_type: RecycleBinItemType::Note,
        payload: Paragraph(1..2).fake(),
        workspace_identifier: Some(meta.workspace_identifier),
    };

    let created = repo.store(&payload, &Some(meta.clone())).await?;

    repo.purge(&created.identifier, &Some(meta.clone()))
        .await?;

    let found = repo
        .find_by_id(&created.identifier, &Some(meta.clone()))
        .await?;

    assert!(found.is_none());

    Ok(())
}

#[tokio::test]
async fn test_purge_all_recycle_bin() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_recycle_bin_repository).await?;

    for _ in 0..3 {
        let payload = CreateRecycleBinEntry {
            item_id: Uuid::new_v4(),
            item_type: RecycleBinItemType::Note,
            payload: Paragraph(1..2).fake(),
            workspace_identifier: Some(meta.workspace_identifier),
        };

        repo.store(&payload, &Some(meta.clone())).await?;
    }

    repo.purge_all(&Some(meta.clone())).await?;

    let results = repo.find_all(&Some(meta.clone())).await?;

    assert!(results.is_empty());

    Ok(())
}