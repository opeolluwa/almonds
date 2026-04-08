use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("wait_list")
                    .if_not_exists()
                    .col(string("identifier").not_null().primary_key())
                    .col(string("first_name").not_null())
                    .col(string("last_name").not_null())
                    .col(string("email").not_null())
                    .col(
                        timestamp("created_at")
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(timestamp_null("updated_at"))
                    .col(timestamp_null("deleted_at"))
                    .col(timestamp_null("last_notified_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("wait_list").to_owned())
            .await
    }
}
