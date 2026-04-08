use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_reference_email_users")
                    .from_tbl("reference_emails")
                    .from_col("user_identifier")
                    .to_tbl("users")
                    .to_col("identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_reference_email_users")
                    .table("reference_emails")
                    .to_owned(),
            )
            .await
    }
}
