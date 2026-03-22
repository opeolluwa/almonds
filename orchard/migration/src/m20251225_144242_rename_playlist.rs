use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let stmt = Table::rename().table("playlist", "playlists").to_owned();
        manager.rename_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let stmt = Table::rename().table("playlists", "playlist").to_owned();
        manager.rename_table(stmt).await
    }
}
