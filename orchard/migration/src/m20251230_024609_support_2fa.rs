use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::alter()
            .add_column(
                ColumnDef::new("enable_2fa")
                    .boolean()
                    .default(false)
                    .not_null(),
            )
            .table("users")
            .to_owned();

        manager.alter_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::alter()
            .drop_column("enable_2fa")
            .table("users")
            .to_owned();

        manager.alter_table(stmt).await
    }
}
