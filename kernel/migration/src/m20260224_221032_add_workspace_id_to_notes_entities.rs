use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

use crate::{
    m20260218_110352_create_note_table::Notes, m20260224_214545_create_workspaces::Workspaces,
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
                    CREATE TABLE IF NOT EXISTS "notes_new" (
                        "identifier" TEXT PRIMARY KEY,
                        "title" TEXT NOT NULL,
                        "content" TEXT NOT NULL,
                        "created_at" TIMESTAMP NOT NULL,
                        "updated_at" TIMESTAMP NOT NULL,
                        "workspace_identifier" UUID,
                        FOREIGN KEY("workspace_identifier") REFERENCES "workspaces"("identifier") ON DELETE CASCADE
                    );
                    INSERT INTO "notes_new" ("identifier", "title", "content", "created_at", "updated_at")
                    SELECT "identifier", "title", "content", "created_at", "updated_at" FROM "notes";
                    DROP TABLE "notes";
                    ALTER TABLE "notes_new" RENAME TO "notes";
                    "#,
                )
                .await?;

            return Ok(());
        }
        manager
            .alter_table(
                Table::alter()
                    .table(Notes::Table)
                    .add_column(ColumnDef::new("workspace_identifier").uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_notes_workspace_identifier")
                    .from(Notes::Table, "workspace_identifier")
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
                    .name("fk_notes_workspace_identifier")
                    .table(Notes::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Notes::Table)
                    .drop_column("workspace_identifier")
                    .to_owned(),
            )
            .await
    }
}
