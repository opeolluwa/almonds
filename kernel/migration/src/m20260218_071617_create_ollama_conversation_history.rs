use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OllamaConversationHistory::Table)
                    .if_not_exists()
                    .col(pk_uuid(OllamaConversationHistory::Identifier))
                    .col(boolean(OllamaConversationHistory::Bookmarked).default(false))
                    .col(timestamp_with_time_zone(
                        OllamaConversationHistory::CreatedAt,
                    ))
                    .col(timestamp_with_time_zone(
                        OllamaConversationHistory::UpdatedAt,
                    ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OllamaConversationHistory::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum OllamaConversationHistory {
    Table,
    Identifier,
    Bookmarked,
    CreatedAt,
    UpdatedAt,
}
