use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NoteCategories::Table)
                    .if_not_exists()
                    .col(pk_uuid(NoteCategories::Identifier))
                    .col(string(NoteCategories::Label))
                    .col(text_null(NoteCategories::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NoteCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NoteCategories {
    Table,
    Identifier,
    Label,
    Description,
}
