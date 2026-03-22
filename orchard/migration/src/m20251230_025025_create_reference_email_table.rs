use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("reference_emails")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("identifier")
                            .string()
                            .string_len(26)
                            .primary_key()
                            .not_null(),
                    )
                    .col(string("email").not_null())
                    .col(uuid("user_identifier").not_null())
                    .col(string("verified").boolean().not_null().default(false))
                    .col(
                        timestamp("created_at")
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(timestamp_null("updated_at"))
                    .col(timestamp_null("deleted_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("reference_emails").to_owned())
            .await
    }
}
