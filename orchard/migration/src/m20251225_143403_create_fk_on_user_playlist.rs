use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let query = include_str!("sqlx/20250718221741_add_user_identifier_to_playlist_table.sql");
        let db = manager.get_connection();

        db.execute_unprepared(query).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let foreign_key = ForeignKey::drop()
            .name("fk_playlist_users")
            .table("playlist")
            .to_owned();

        manager.drop_foreign_key(foreign_key.to_owned()).await
    }
}
