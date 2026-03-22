use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let query = include_str!("sqlx/20250713195059_add_playlist_identifier_to_audio_books.sql");
        let db = manager.get_connection();

        db.execute_unprepared(query).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let foreign_key = ForeignKey::drop()
            .name("fk_audio_books_playlist")
            .table("audio_books")
            .to_owned();

        manager.drop_foreign_key(foreign_key.to_owned()).await
    }
}
