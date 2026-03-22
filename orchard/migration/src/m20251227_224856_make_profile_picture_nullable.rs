use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::alter()
            .table("users")
            .modify_column(
                ColumnDef::new("profile_picture")
                    .string()
                    .null()
                    .default(None::<String>),
            )
            .to_owned();

        manager.alter_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::alter()
            .table("users")
            .modify_column(ColumnDef::new("profile_picture").string().not_null())
            .to_owned();

        manager.alter_table(stmt).await
    }
}
