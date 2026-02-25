use sea_orm_migration::{prelude::*, schema::*, sea_orm::DbBackend};

use crate::{
    m20260218_204212_create_bookmarks_table::{Bookmark, Tag},
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
                        .table("bookmarks_new")
                        .if_not_exists()
                        .col(pk_uuid("identifier"))
                        .col(string("title"))
                        .col(string("url"))
                        .col(
                            ColumnDef::new("tag")
                                .enumeration(
                                    Tag::Type,
                                    [
                                        Tag::Development,
                                        Tag::Inspiration,
                                        Tag::Design,
                                        Tag::Research,
                                    ],
                                )
                                .not_null()
                                .default("development"),
                        )
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_bookmark_workspace_identifier")
                                .from(Bookmark::Table, "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(timestamp_with_time_zone(Bookmark::CreatedAt))
                        .col(timestamp_with_time_zone(Bookmark::UpdatedAt))
                        .to_owned(),
                )
                .await?;

            db_connection
                .execute_unprepared(
                    r#"
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
