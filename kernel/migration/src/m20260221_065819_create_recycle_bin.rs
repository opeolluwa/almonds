use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecycleBin::Table)
                    .if_not_exists()
                    .col(pk_uuid(RecycleBin::Identifier))
                    .col(uuid(RecycleBin::ItemId))
                    .col(
                        ColumnDef::new(RecycleBin::ItemType)
                            .enumeration(
                                ItemType::Type,
                                [
                                    ItemType::Todo,
                                    ItemType::Note,
                                    ItemType::Reminder,
                                    ItemType::Snippet,
                                    ItemType::Bookmark,
                                ],
                            )
                            .not_null(),
                    )
                    .col(text(RecycleBin::Payload))
                    .col(timestamp_with_time_zone(RecycleBin::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecycleBin::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RecycleBin {
    Table,
    Identifier,
    ItemId,
    ItemType,
    Payload,
    DeletedAt,
}

#[derive(DeriveIden)]
enum ItemType {
    #[sea_orm(iden = "item_type")]
    Type,
    Todo,
    Note,
    Reminder,
    Snippet,
    Bookmark,
}
