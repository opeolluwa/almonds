use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260218_110352_create_note_table::Notes, m20260224_214545_create_workspaces::Workspaces,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("notes_new").if_exists().to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("notes_new")
                    .if_not_exists()
                    .col(pk_uuid("identifier"))
                    .col(string("title"))
                    .col(text("content"))
                    .col(json_null("categories"))
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
            .await
    }
}
