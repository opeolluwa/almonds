use sea_orm_migration::{prelude::*, schema::*};

use super::m20260218_071617_create_ollama_conversation_history::OllamaConversationHistory;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OllamaConversationResponse::Table)
                    .if_not_exists()
                    .col(pk_uuid(OllamaConversationResponse::Identifier))
                    .col(uuid(OllamaConversationResponse::HistoryId))
                    .col(text(OllamaConversationResponse::Content))
                    .col(timestamp_with_time_zone(
                        OllamaConversationResponse::CreatedAt,
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_response_history")
                            .from(
                                OllamaConversationResponse::Table,
                                OllamaConversationResponse::HistoryId,
                            )
                            .to(
                                OllamaConversationHistory::Table,
                                OllamaConversationHistory::Identifier,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OllamaConversationResponse::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum OllamaConversationResponse {
    Table,
    Identifier,
    HistoryId,
    Content,
    CreatedAt,
}
