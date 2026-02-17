use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Snippets::Table)
                    .if_not_exists()
                    .col(pk_uuid(Snippets::Identifier))
                    .col(string_null(Snippets::Title))
                    .col(string_null(Snippets::Language))
                    .col(text(Snippets::Code))
                    .col(text_null(Snippets::Description))
                    .col(boolean(Snippets::IsPinned).default(false))
                    .col(timestamp_with_time_zone(Snippets::CreatedAt))
                    .col(timestamp_with_time_zone(Snippets::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Snippets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Snippets {
    Table,
    Identifier,
    Title,
    Language,
    Code,
    Description,
    IsPinned,
    CreatedAt,
    UpdatedAt,
}
