use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::DbBackend,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();

        if db_backend == DbBackend::Postgres {
            manager
                .create_type(
                    Type::create()
                        .as_enum(Priority::Type)
                        .values([Priority::High, Priority::Medium, Priority::Low])
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(pk_uuid(Todo::Identifier))
                    .col(string(Todo::Title))
                    .col(text_null(Todo::Description))
                    .col(date_null(Todo::DueDate))
                    .col(
                        ColumnDef::new(Todo::Priority)
                            .enumeration(
                                Priority::Type,
                                [Priority::High, Priority::Medium, Priority::Low],
                            )
                            .not_null()
                            .default("medium"),
                    )
                    .col(boolean(Todo::Done).default(false))
                    .col(timestamp_with_time_zone(Todo::CreatedAt))
                    .col(timestamp_with_time_zone(Todo::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Todo {
    Table,
    Identifier,
    Title,
    Description,
    DueDate,
    Priority,
    Done,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Priority {
    #[sea_orm(iden = "priority")]
    Type,
    High,
    Medium,
    Low,
}
