use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let query = include_str!("sqlx/20251204062241_add_country_to_user.sql");
        let db = manager.get_connection();

        db.execute_unprepared(query).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let stmt = Table::alter()
            .drop_column("country_identifier")
            .table("users")
            .to_owned();

        manager.alter_table(stmt).await
    }
}
