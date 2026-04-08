use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let stmt = Table::alter()
            .add_column(string("profile_picture"))
            .table("users")
            .to_owned();

        manager.alter_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let stmt = Table::alter()
            .drop_column("profile_picture")
            .table("users")
            .to_owned();

        manager.alter_table(stmt).await
    }
}
