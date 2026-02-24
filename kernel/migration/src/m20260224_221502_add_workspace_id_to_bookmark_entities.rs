use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

use crate::{m20260218_204212_create_bookmarks_table::Bookmark, m20260224_214545_create_workspaces::Workspaces};

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
                    CREATE TABLE IF NOT EXISTS "bookmarks_new" (
                        "identifier" UUID PRIMARY KEY,
                        "title" TEXT NOT NULL,
                        "url" TEXT NOT NULL,
                        "created_at" TIMESTAMP NOT NULL,
                        "updated_at" TIMESTAMP NOT NULL,
                        "workspace_identifier" UUID,
                        FOREIGN KEY("workspace_identifier") REFERENCES "workspaces"("identifier") ON DELETE CASCADE
                    );
                    INSERT INTO "bookmarks_new" ("identifier", "title", "url", "created_at", "updated_at")
                    SELECT "identifier", "title", "url", "created_at", "updated_at" FROM "bookmark";
                    DROP TABLE "bookmark";
                    ALTER TABLE "bookmarks_new" RENAME TO "bookmark";
                    "#,
                )
                .await?;

            return Ok(());
        }
        manager
            .alter_table(
                Table::alter()
                    .table(Bookmark::Table)
                    .add_column(ColumnDef::new("workspace_identifier").uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_bookmark_workspace_identifier")
                    .from(Bookmark::Table, "workspace_identifier")
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
                    .name("fk_bookmark_workspace_identifier")
                    .table(Bookmark::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Bookmark::Table)
                    .drop_column("workspace_identifier")
                    .to_owned(),
            )
            .await
    }
}
