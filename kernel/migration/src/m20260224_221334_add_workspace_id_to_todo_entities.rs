use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260218_171131_create_todo_table::Todo, m20260224_214545_create_workspaces::Workspace,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Todo::Table)
                    .add_column(ColumnDef::new("workspace_identifier").uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_todo_workspace_identifier")
                    .from(Todo::Table, "workspace_identifier")
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
                    .name("fk_todo_workspace_identifier")
                    .table(Todo::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Todo::Table)
                    .drop_column("workspace_identifier")
                    .to_owned(),
            )
            .await
    }
}
