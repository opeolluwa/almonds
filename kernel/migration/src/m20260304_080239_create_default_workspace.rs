use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    //     let db_connection = manager.get_connection();

    //     let workspace_identifier = Uuid::new_v4();

    //     db_connection
    //         .execute_unprepared(&format!(
    //             r#"
    //     INSERT INTO workspaces (identifier, name, description, created_at, updated_at)
    //     SELECT '{}', 'default', 'Default workspace', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
    //     WHERE NOT EXISTS (
    //         SELECT 1 FROM workspaces WHERE name = 'default'
    //     );
    //     "#,
    //             workspace_identifier
    //         ))
    //         .await?;

    //     db_connection
    //         .execute_unprepared(&format!(
    //             r#"
    //     UPDATE todo SET workspace_identifier = '{0}' WHERE workspace_identifier IS NULL;
    //     UPDATE notes SET workspace_identifier = '{0}' WHERE workspace_identifier IS NULL;
    //     UPDATE bookmark SET workspace_identifier = '{0}' WHERE workspace_identifier IS NULL;
    //     UPDATE recycle_bin SET workspace_identifier = '{0}' WHERE workspace_identifier IS NULL;
    //     UPDATE reminder SET workspace_identifier = '{0}' WHERE workspace_identifier IS NULL;
    //     UPDATE snippets SET workspace_identifier = '{0}' WHERE workspace_identifier IS NULL;
    //     "#,
    //             workspace_identifier
    //         ))
    //         .await?;
    //     Ok(())
    // }

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_connection = manager.get_connection();
        let workspace_identifier = Uuid::new_v4();

        // Use the raw bytes, formatted as a SQLite blob literal X'...'
        let uuid_hex = hex::encode(workspace_identifier.as_bytes());

        db_connection
            .execute_unprepared(&format!(
                r#"
                INSERT INTO workspaces (identifier, name, description, created_at, updated_at)
                SELECT X'{0}', 'default', 'Default workspace', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                WHERE NOT EXISTS (
                    SELECT 1 FROM workspaces WHERE name = 'default'
                );
                "#,
                uuid_hex
            ))
            .await?;

        db_connection
            .execute_unprepared(&format!(
                r#"
                UPDATE todo SET workspace_identifier = X'{0}' WHERE workspace_identifier IS NULL;
                UPDATE notes SET workspace_identifier = X'{0}' WHERE workspace_identifier IS NULL;
                UPDATE bookmark SET workspace_identifier = X'{0}' WHERE workspace_identifier IS NULL;
                UPDATE recycle_bin SET workspace_identifier = X'{0}' WHERE workspace_identifier IS NULL;
                UPDATE reminder SET workspace_identifier = X'{0}' WHERE workspace_identifier IS NULL;
                UPDATE snippets SET workspace_identifier = X'{0}' WHERE workspace_identifier IS NULL;
                "#,
                uuid_hex
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_connection = manager.get_connection();

        // Reset workspace_identifier back to NULL
        db_connection
            .execute_unprepared(
                r#"
            UPDATE todo SET workspace_identifier = NULL;
            UPDATE notes SET workspace_identifier = NULL;
            UPDATE bookmark SET workspace_identifier = NULL;
            UPDATE recycle_bin SET workspace_identifier = NULL;
            UPDATE reminder SET workspace_identifier = NULL;
            UPDATE snippets SET workspace_identifier = NULL;
            "#,
            )
            .await?;

        db_connection
            .execute_unprepared(
                r#"
        DELETE FROM workspaces
        WHERE name = 'default';
        "#,
            )
            .await?;

        Ok(())
    }
}
