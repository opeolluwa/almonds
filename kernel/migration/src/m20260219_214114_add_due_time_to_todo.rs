use sea_orm_migration::{prelude::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let db_backend = manager.get_database_backend();
        if db_backend == DatabaseBackend::MySql {
            manager
                .alter_table(
                    Table::alter()
                        .table("todo")
                        .add_column(ColumnDef::new("due_time").time().null())
                        .to_owned(),
                )
                .await
        } else {
            manager
                .alter_table(
                    Table::alter()
                        .table("todo")
                        .add_column_if_not_exists(ColumnDef::new("due_time").time().null())
                        .to_owned(),
                )
                .await
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table("todo")
                    .drop_column("due_time")
                    .to_owned(),
            )
            .await
    }
}
