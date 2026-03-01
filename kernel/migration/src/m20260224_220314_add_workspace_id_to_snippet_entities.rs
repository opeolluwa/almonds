use crate::{
    m20260217_143820_create_snippet_table::Snippets, m20260224_214545_create_workspaces::Workspaces,
};
use sea_orm_migration::{
    prelude::*,
    schema::{boolean, pk_uuid, string_null, text, text_null, timestamp_with_time_zone},
    sea_orm::DbBackend,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();
        if db_backend == DbBackend::Sqlite {
            // SQLite does not support adding a foreign key constraint to an existing table.
            // We need to create a new table with the desired schema, copy the data, drop the old table, and rename the new table.
            manager
                .create_table(
                    Table::create()
                        .table("snippets_new")
                        .if_not_exists()
                        .col(pk_uuid("identifier"))
                        .col(string_null("title"))
                        .col(string_null("language"))
                        .col(text("code"))
                        .col(text_null("description"))
                        .col(boolean("is_pinned").default(false))
                        .col(timestamp_with_time_zone("created_at"))
                        .col(timestamp_with_time_zone("updated_at"))
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_snippets_workspace_identifier")
                                .from(Snippets::Table, "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;

            db_connection
                .execute_unprepared(
                    r#"
                    INSERT INTO "snippets_new" ("identifier", "title", "code", "description", "language", "is_pinned", "created_at", "updated_at")

                    SELECT "identifier", "title", "code", "description", "language", "is_pinned", "created_at", "updated_at" FROM "snippets";
                    DROP TABLE "snippets";
                    ALTER TABLE "snippets_new" RENAME TO "snippets";
                    "#,
                )
                .await?;

            return Ok(());
        }

        manager
            .alter_table(
                Table::alter()
                    .table(Snippets::Table)
                    .add_column(ColumnDef::new("workspace_identifier").uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_snippets_workspace_identifier")
                    .from(Snippets::Table, "workspace_identifier")
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
                    .name("fk_snippets_workspace_identifier")
                    .table(Snippets::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Snippets::Table)
                    .drop_column("workspace_identifier")
                    .to_owned(),
            )
            .await
    }
}
