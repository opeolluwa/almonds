use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

use crate::{
    m20260221_065202_create_reminder_table::Reminder, m20260224_214545_create_workspaces::Workspaces,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

           let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();
        if db_backend == DbBackend::Sqlite {
            // For SQLite, we need to create a new table with the workspace_identifier column, copy the data, and then replace the old table
            db_connection
                .execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "reminders_new" (
                        "identifier" TEXT PRIMARY KEY,
                        "title" TEXT NOT NULL,
                        "description" TEXT,
                        "recurring" BOOLEAN NOT NULL DEFAULT FALSE,
                        "recurrence_rule" TEXT,
                        "alarm_sound" TEXT,
                        "remind_at" TIMESTAMP WITH TIME ZONE,
                        "created_at" TIMESTAMP WITH TIME ZONE,
                        "updated_at" TIMESTAMP WITH TIME ZONE,
                        "workspace_identifier" UUID,
                        FOREIGN KEY("workspace_identifier") REFERENCES "workspaces"("identifier") ON DELETE CASCADE
                    );
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
