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
        } else if backend == DbBackend::MySql {
            // Drop the broken trigger from m20260314 (uses UUID() → 36-char string
            // into BINARY(16)) before any workspace UPDATE, then recreate it correctly.
            // m20260501 will later reconcile all other triggers.
            db.execute_unprepared("DROP TRIGGER IF EXISTS workspaces_sync_update")
                .await?;

            if !manager.has_column("workspaces", "is_default").await? {
                db.execute_unprepared(
                    "ALTER TABLE workspaces ADD COLUMN is_default TINYINT NOT NULL DEFAULT 0",
                )
                .await?;
            }
            if !manager.has_column("workspaces", "is_hidden").await? {
                db.execute_unprepared(
                    "ALTER TABLE workspaces ADD COLUMN is_hidden TINYINT NOT NULL DEFAULT 0",
                )
                .await?;
            }
            db.execute_unprepared("UPDATE workspaces SET is_default = 1 WHERE name = 'default'")
                .await?;
            db.execute_unprepared(
                "CREATE TRIGGER workspaces_sync_update \
                 AFTER UPDATE ON workspaces FOR EACH ROW \
                 INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at) \
                 VALUES (UUID_TO_BIN(UUID()), 'workspaces', NEW.identifier, 'UPDATE', NOW())",
            )
            .await?;
        } else if backend == DbBackend::Postgres {
            db.execute_unprepared(
                r#"
                ALTER TABLE workspaces ADD COLUMN IF NOT EXISTS is_default BOOLEAN NOT NULL DEFAULT FALSE;
                ALTER TABLE workspaces ADD COLUMN IF NOT EXISTS is_hidden BOOLEAN NOT NULL DEFAULT FALSE;
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
