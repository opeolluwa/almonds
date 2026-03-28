use sea_orm_migration::{prelude::*, schema::*, sea_orm::DbBackend};

use crate::{
    m20260221_065938_create_user_preference_table::UserPreference,
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
                        .table("user_preference_new")
                        .if_not_exists()
                        .col(pk_uuid("identifier"))
                        .col(string("first_name"))
                        .col(string("last_name"))
                        .col(string_uniq("email"))
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_user_preference_workspace_identifier")
                                .from(UserPreference::Table, "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(timestamp_with_time_zone(UserPreference::CreatedAt))
                        .col(timestamp_with_time_zone(UserPreference::UpdatedAt))
                        .to_owned(),
                )
                .await?;

            db_connection
                .execute_unprepared(
                    r#"
                    INSERT INTO "user_preference_new" ("identifier", "first_name", "last_name", "email", "created_at", "updated_at")
                    SELECT "identifier", "first_name", "last_name", "email", "created_at", "updated_at" FROM "user_preference";

                    DROP TABLE "user_preference";
                    ALTER TABLE "user_preference_new" RENAME TO "user_preference";
                    "#,
                )
                .await?;

            return Ok(());
        }

        manager
            .alter_table(
                Table::alter()
                    .table(UserPreference::Table)
                    .add_column(ColumnDef::new("workspace_identifier").uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_preference_workspace_identifier")
                    .from(UserPreference::Table, "workspace_identifier")
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
                    .name("fk_user_preference_workspace_identifier")
                    .table(UserPreference::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserPreference::Table)
                    .drop_column("workspace_identifier")
                    .to_owned(),
            )
            .await
    }
}
