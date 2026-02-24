use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(pk_uuid(Workspace::Identifier))
                    .col(string(Workspace::Name))
                    .col(string(Workspace::Description))
                    .col(timestamp_with_time_zone(Workspace::CreatedAt))
                    .col(timestamp_with_time_zone(Workspace::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Workspace {
    Table,
    Identifier,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}
