use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let stmt = Table::alter()
            .table("one_time_password")
            .rename_column("otp", "code")
            .to_owned();
        manager.alter_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let stmt = Table::alter()
            .table("one_time_password")
            .rename_column("code", "otp")
            .to_owned();
        manager.alter_table(stmt).await
    }
}
