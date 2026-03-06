use sea_orm_migration::{prelude::*, schema::*, sea_orm::DbBackend};

use crate::{
    m20260221_065202_create_reminder_table::Reminder,
    m20260224_214545_create_workspaces::Workspaces,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();
        if db_backend == DbBackend::Sqlite {
            manager
                .create_table(
                    Table::create()
                        .table("reminders_new")
                        .if_not_exists()
                        .col(pk_uuid(Reminder::Identifier))
                        .col(string(Reminder::Title))
                        .col(text_null(Reminder::Description))
                        .col(boolean(Reminder::Recurring).default(false))
                        .col(string_null(Reminder::RecurrenceRule))
                        .col(string_null(Reminder::AlarmSound))
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_reminder_workspace_identifier")
                                .from(Reminder::Table, "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(timestamp_with_time_zone(Reminder::RemindAt))
                        .col(timestamp_with_time_zone(Reminder::CreatedAt))
                        .col(timestamp_with_time_zone(Reminder::UpdatedAt))
                        .to_owned(),
                )
                .await?;

            db_connection
                .execute_unprepared(
                    r#"
                    INSERT INTO "reminders_new" ("identifier", "title", "description", "recurring", "recurrence_rule", "alarm_sound", "remind_at", "created_at", "updated_at")

                    SELECT "identifier", "title", "description", "recurring", "recurrence_rule", "alarm_sound", "remind_at", "created_at", "updated_at" FROM "reminder";
                    
                    DROP TABLE "reminder";

                    ALTER TABLE "reminders_new" RENAME TO "reminder";
                    "#,
                )
                .await?;
            return Ok(());
        }

        manager
            .alter_table(
                Table::alter()
                    .table(Reminder::Table)
                    .add_column(ColumnDef::new("workspace_identifier").uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_reminder_workspace_identifier")
                    .from(Reminder::Table, "workspace_identifier")
                    .to(Workspaces::Table, "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_reminder_workspace_identifier")
                    .table(Reminder::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Reminder::Table)
                    .drop_column("workspace_identifier")
                    .to_owned(),
            )
            .await
    }
}
