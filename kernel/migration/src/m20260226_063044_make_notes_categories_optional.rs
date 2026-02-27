use sea_orm_migration::{prelude::*, schema::*, sea_orm::DbBackend};

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
            manager
                .create_table(
                    Table::create()
                        .table("notes_new")
                        .if_not_exists()
                        .col(pk_uuid("identifier"))
                        .col(string("title"))
                        .col(text("content"))
                        .col(json("categories").null())
                        .col(timestamp_with_time_zone("created_at"))
                        .col(timestamp_with_time_zone("updated_at"))
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_notes_workspace_identifier")
                                .from(Notes::Table, "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;

            db_connection
                 .execute_unprepared(
                     r#"
                     INSERT INTO "notes_new" ("identifier", "title", "content", "categories", "created_at", "updated_at")

                     SELECT "identifier", "title", "content", "categories", "created_at", "updated_at" FROM "notes";

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
                    .modify_column(ColumnDef::new(Notes::Categories).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();
        if db_backend == DbBackend::Sqlite {
            manager
                .create_table(
                    Table::create()
                        .table("notes_new")
                        .if_not_exists()
                        .col(pk_uuid("identifier"))
                        .col(string("title"))
                        .col(text("content"))
                        .col(json("categories").not_null())
                        .col(timestamp_with_time_zone("created_at"))
                        .col(timestamp_with_time_zone("updated_at"))
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_notes_workspace_identifier")
                                .from(Notes::Table, "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;

            db_connection
                 .execute_unprepared(
                     r#"
                     INSERT INTO "notes_new" ("identifier", "title", "content", "categories", "created_at", "updated_at")

                     SELECT "identifier", "title", "content", "categories", "created_at", "updated_at" FROM "notes";

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
                    .modify_column(ColumnDef::new(Notes::Categories).not_null())
                    .to_owned(),
            )
            .await
    }
}
