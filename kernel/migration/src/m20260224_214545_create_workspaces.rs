use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Workspaces::Table)
                    .if_not_exists()
                    .col(pk_uuid(Workspaces::Identifier))
                    .col(string(Workspaces::Name))
                    .col(string(Workspaces::Description))
                    .col(timestamp_with_time_zone(Workspaces::CreatedAt))
                    .col(timestamp_with_time_zone(Workspaces::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Workspaces::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Workspaces {
    Table,
    Identifier,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}
