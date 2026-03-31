use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        if backend == DbBackend::Sqlite {
            db.execute_unprepared(
                r#"
                ALTER TABLE workspaces ADD COLUMN is_default BOOLEAN NOT NULL DEFAULT FALSE;
                ALTER TABLE workspaces ADD COLUMN is_hidden BOOLEAN NOT NULL DEFAULT FALSE;
                UPDATE workspaces SET is_default = TRUE WHERE name = 'default';
                "#,
            )
            .await?;
        } else {
            db.execute_unprepared(
                r#"
                ALTER TABLE workspaces ADD COLUMN is_default BOOLEAN NOT NULL DEFAULT FALSE;
                ALTER TABLE workspaces ADD COLUMN is_hidden BOOLEAN NOT NULL DEFAULT FALSE;
                UPDATE workspaces SET is_default = TRUE WHERE name = 'default';
                "#,
            )
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        if backend == DbBackend::Sqlite {
            db.execute_unprepared(
                r#"
                CREATE TABLE workspaces_backup (
                    identifier BLOB PRIMARY KEY NOT NULL,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );
                INSERT INTO workspaces_backup SELECT identifier, name, description, created_at, updated_at FROM workspaces;
                DROP TABLE workspaces;
                ALTER TABLE workspaces_backup RENAME TO workspaces;
                "#,
            )
            .await?;
        } else {
            db.execute_unprepared(
                r#"
                ALTER TABLE workspaces DROP COLUMN is_default;
                ALTER TABLE workspaces DROP COLUMN is_hidden;
                "#,
            )
            .await?;
        }

        Ok(())
    }
}
