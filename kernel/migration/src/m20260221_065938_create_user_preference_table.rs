use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPreference::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserPreference::Identifier))
                    .col(string(UserPreference::FirstName))
                    .col(string(UserPreference::LastName))
                    .col(string_uniq(UserPreference::Email))
                    .col(timestamp_with_time_zone(UserPreference::CreatedAt))
                    .col(timestamp_with_time_zone(UserPreference::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPreference::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserPreference {
    Table,
    Identifier,
    FirstName,
    LastName,
    Email,
    CreatedAt,
    UpdatedAt,
}
