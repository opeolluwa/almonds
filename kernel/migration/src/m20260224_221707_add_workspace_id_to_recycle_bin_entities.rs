use sea_orm_migration::{prelude::*, schema::*, sea_orm::DbBackend};

use crate::{
    m20260221_065819_create_recycle_bin::{ItemType, RecycleBin},
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
                        .table("recycle_bin_new")
                        .if_not_exists()
                        .col(pk_uuid(RecycleBin::Identifier))
                        .col(uuid(RecycleBin::ItemId))
                        .col(
                            ColumnDef::new(RecycleBin::ItemType)
                                .enumeration(
                                    ItemType::Type,
                                    [
                                        ItemType::Todo,
                                        ItemType::Note,
                                        ItemType::Reminder,
                                        ItemType::Snippet,
                                        ItemType::Bookmark,
                                    ],
                                )
                                .not_null(),
                        )
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_recycle_bin_workspace_identifier")
                                .from(RecycleBin::Table, "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(text(RecycleBin::Payload))
                        .col(timestamp_with_time_zone(RecycleBin::DeletedAt))
                        .to_owned(),
                )
                .await?;

            db_connection
                .execute_unprepared(
                    r#"
                    INSERT INTO "recycle_bin_new" ("identifier", "item_id", "item_type", "payload", "deleted_at")

                    SELECT "identifier", "item_id", "item_type", "payload", "deleted_at" FROM "recycle_bin";

                    DROP TABLE "recycle_bin";

                    ALTER TABLE "recycle_bin_new" RENAME TO "recycle_bin";
                    "#,
                )
                .await?;
            return Ok(());
        }

        manager
            .alter_table(
                Table::alter()
                    .table(RecycleBin::Table)
                    .add_column(ColumnDef::new("workspace_identifier").uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_recycle_bin_workspace_identifier")
                    .from(RecycleBin::Table, "workspace_identifier")
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
                    .name("fk_recycle_bin_workspace_identifier")
                    .table(RecycleBin::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(RecycleBin::Table)
                    .drop_column("workspace_identifier")
                    .to_owned(),
            )
            .await
    }
}
