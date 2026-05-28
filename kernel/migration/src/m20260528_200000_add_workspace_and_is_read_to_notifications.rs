use sea_orm_migration::{prelude::*, schema::*, sea_orm::DbBackend};

use crate::{
    m20260224_214545_create_workspaces::Workspaces, m20260528_132342_notification::NotificationType,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();

        if db_backend == DbBackend::Sqlite {
            db_connection
                .execute_unprepared(
                    r#"
                    DROP TABLE "notifications";             
                    "#,
                )
                .await?;

            manager
                .create_table(
                    Table::create()
                        .table("notifications")
                        .if_not_exists()
                        .col(pk_uuid("identifier"))
                        .col(string("title"))
                        .col(string("body"))
                        .col(
                            enumeration(
                                "notification_type",
                                "notification_type",
                                [
                                    NotificationType::BackupFailed,
                                    NotificationType::BackupSuccess,
                                    NotificationType::WorkspaceInviteReceived,
                                    NotificationType::WorkspaceInviteAccepted,
                                    NotificationType::WorkspaceInviteDeclined,
                                    NotificationType::ItemShared,
                                    NotificationType::ItemUnshared,
                                    NotificationType::ItemUpdated,
                                    NotificationType::ItemDeleted,
                                    NotificationType::ItemAccessGranted,
                                    NotificationType::ItemAccessRevoked,
                                    NotificationType::Generic,
                                ],
                            )
                            .not_null()
                            .default("generic".to_string()),
                        )
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_notification_workspace_identifier")
                                .from("notifications", "workspace_identifier")
                                .to(Workspaces::Table, "identifier")
                                .on_delete(ForeignKeyAction::Cascade),
                        )
                        .col(
                            ColumnDef::new("is_read")
                                .boolean()
                                .not_null()
                                .default(false),
                        )
                        .col(ColumnDef::new("workspace_identifier").uuid())
                        .col(timestamp_with_time_zone("created_at"))
                        .col(timestamp_with_time_zone("updated_at"))
                        .to_owned(),
                )
                .await?;

            return Ok(());
        }

        let workspace_col = if db_backend == DbBackend::MySql {
            ColumnDef::new("workspace_identifier")
                .string_len(36)
                .to_owned()
        } else {
            ColumnDef::new("workspace_identifier").uuid().to_owned()
        };

        manager
            .alter_table(
                Table::alter()
                    .table("notifications")
                    .add_column_if_not_exists(
                        ColumnDef::new("is_read")
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column_if_not_exists(workspace_col)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_notification_workspace_identifier")
                    .from("notifications", "workspace_identifier")
                    .to(Workspaces::Table, "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();

        if db_backend != DbBackend::Sqlite {
            manager
                .drop_foreign_key(
                    ForeignKey::drop()
                        .name("fk_notification_workspace_identifier")
                        .table("notifications")
                        .to_owned(),
                )
                .await?;
        }

        manager
            .alter_table(
                Table::alter()
                    .table("notifications")
                    .drop_column("is_read")
                    .drop_column("workspace_identifier")
                    .to_owned(),
            )
            .await
    }
}
