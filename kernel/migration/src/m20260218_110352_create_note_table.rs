use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notes::Table)
                    .if_not_exists()
                    .col(pk_uuid(Notes::Identifier))
                    .col(string(Notes::Title))
                    .col(text(Notes::Content))
                    .col(json(Notes::Categories))
                    .col(timestamp_with_time_zone(Notes::CreatedAt))
                    .col(timestamp_with_time_zone(Notes::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Notes {
    Table,
    Identifier,
    Title,
    Content,
    Categories,
    CreatedAt,
    UpdatedAt,
}
