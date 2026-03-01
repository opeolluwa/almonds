use sea_orm_migration::{prelude::*, schema::*, sea_orm::DbBackend};

use crate::{
    m20260218_171131_create_todo_table::{Priority, Todo},
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
                        .table("todo_new")
                        .if_not_exists()
                        .col(pk_uuid("identifier"))
                        .col(string("title"))
                        .col(text_null("description"))
                        .col(date_null("due_date"))
                        .col(
                            ColumnDef::new("priority")
                                .enumeration(
                                    Priority::Type,
                                    [Priority::High, Priority::Medium, Priority::Low],
                                )
                                .not_null()
                                .default("medium"),
                        )
                        .col(timestamp_with_time_zone("created_at"))
                        .col(timestamp_with_time_zone("updated_at"))
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .col(boolean("done").default(false))
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_todo_workspace_identifier")
                                .from(Todo::Table, "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;

            db_connection
                 .execute_unprepared(
                     r#"
                     INSERT INTO "todo_new" ("identifier", "title", "description", "done", "created_at", "updated_at")

                     SELECT "identifier", "title", "description", "done", "created_at", "updated_at" FROM "todo";

                     DROP TABLE "todo";
                     ALTER TABLE "todo_new" RENAME TO "todo";
                        "#,
                    )
                    .await
                    ?;
            return Ok(());
        }

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
