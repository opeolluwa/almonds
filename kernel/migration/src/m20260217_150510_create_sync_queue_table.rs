use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SyncQueue::Table)
                    .if_not_exists()
                    .col(pk_uuid(SyncQueue::Identifier))
                    .col(string(SyncQueue::TableName))
                    .col(string(SyncQueue::RecordIdentifier))
                    .col(string(SyncQueue::Operation))
                    .col(string(SyncQueue::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SyncQueue::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SyncQueue {
    Table,
    Identifier,
    TableName,
    RecordIdentifier,
    Operation,
    CreatedAt,
}
