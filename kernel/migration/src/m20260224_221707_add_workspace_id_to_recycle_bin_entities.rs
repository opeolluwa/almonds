use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20260221_065819_create_recycle_bin::RecycleBin, m20260224_214545_create_workspaces::Workspace};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .to(Workspace::Table, "identifier")
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
