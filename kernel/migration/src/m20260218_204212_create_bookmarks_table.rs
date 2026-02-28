use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bookmark::Table)
                    .if_not_exists()
                    .col(pk_uuid(Bookmark::Identifier))
                    .col(string(Bookmark::Title))
                    .col(string(Bookmark::Url))
                    .col(
                        ColumnDef::new(Bookmark::Tag)
                            .enumeration(
                                Tag::Type,
                                [
                                    Tag::Development,
                                    Tag::Inspiration,
                                    Tag::Design,
                                    Tag::Research,
                                ],
                            )
                            .not_null()
                            .default("development"),
                    )
                    .col(timestamp_with_time_zone(Bookmark::CreatedAt))
                    .col(timestamp_with_time_zone(Bookmark::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bookmark::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Bookmark {
    Table,
    Identifier,
    Title,
    Url,
    Tag,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Tag {
    #[sea_orm(iden = "tag")]
    Type,
    Development,
    Inspiration,
    Design,
    Research,
}
