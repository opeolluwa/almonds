use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reminder::Table)
                    .if_not_exists()
                    .col(pk_uuid(Reminder::Identifier))
                    .col(string(Reminder::Title))
                    .col(text_null(Reminder::Description))
                    .col(boolean(Reminder::Recurring).default(false))
                    .col(string_null(Reminder::RecurrenceRule))
                    .col(string_null(Reminder::AlarmSound))
                    .col(timestamp_with_time_zone(Reminder::RemindAt))
                    .col(timestamp_with_time_zone(Reminder::CreatedAt))
                    .col(timestamp_with_time_zone(Reminder::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reminder::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Reminder {
    Table,
    Identifier,
    Title,
    Description,
    Recurring,
    RecurrenceRule,
    AlarmSound,
    RemindAt,
    CreatedAt,
    UpdatedAt,
}
