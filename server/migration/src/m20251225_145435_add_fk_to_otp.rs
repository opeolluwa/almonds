use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let query = include_str!(
            "sqlx/20250806155111_update-er_add_on_dele_specifier_to_user_and_on_time_password.sql"
        );
        let db = manager.get_connection();

        db.execute_unprepared(query).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let foreign_key = ForeignKey::drop()
            .name("one_time_password_user_identifier_fkey")
            .table("one_time_passwords")
            .to_owned();

        manager.drop_foreign_key(foreign_key.to_owned()).await
    }
}
