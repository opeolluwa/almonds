use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();

        if db_backend == DbBackend::Sqlite {
            // Old triggers inserted lower(hex(randomblob(16))) — a 32-char text string.
            // sqlx decodes Uuid columns by calling value.blob(), expecting 16 raw bytes.
            // 32 text bytes → "invalid length: expected 16 bytes, found 32".
            // Clear the bad rows and replace triggers with randomblob(16) (16 raw bytes).
            db_connection
                .execute_unprepared("DELETE FROM sync_queue;")
                .await?;

            let drop_triggers = r#"
DROP TRIGGER IF EXISTS bookmark_sync_insert;
DROP TRIGGER IF EXISTS bookmark_sync_update;
DROP TRIGGER IF EXISTS bookmark_sync_delete;
DROP TRIGGER IF EXISTS notes_sync_insert;
DROP TRIGGER IF EXISTS notes_sync_update;
DROP TRIGGER IF EXISTS notes_sync_delete;
DROP TRIGGER IF EXISTS recycle_bin_sync_insert;
DROP TRIGGER IF EXISTS recycle_bin_sync_update;
DROP TRIGGER IF EXISTS recycle_bin_sync_delete;
DROP TRIGGER IF EXISTS reminder_sync_insert;
DROP TRIGGER IF EXISTS reminder_sync_update;
DROP TRIGGER IF EXISTS reminder_sync_delete;
DROP TRIGGER IF EXISTS snippets_sync_insert;
DROP TRIGGER IF EXISTS snippets_sync_update;
DROP TRIGGER IF EXISTS snippets_sync_delete;
DROP TRIGGER IF EXISTS todo_sync_insert;
DROP TRIGGER IF EXISTS todo_sync_update;
DROP TRIGGER IF EXISTS todo_sync_delete;
DROP TRIGGER IF EXISTS workspaces_sync_insert;
DROP TRIGGER IF EXISTS workspaces_sync_update;
DROP TRIGGER IF EXISTS workspaces_sync_delete;
DROP TRIGGER IF EXISTS user_preference_sync_insert;
DROP TRIGGER IF EXISTS user_preference_sync_update;
DROP TRIGGER IF EXISTS user_preference_sync_delete;
"#;
            db_connection.execute_unprepared(drop_triggers).await?;

            let triggers = r#"
-- BOOKMARK
CREATE TRIGGER bookmark_sync_insert
AFTER INSERT ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'bookmark', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
END;

CREATE TRIGGER bookmark_sync_update
AFTER UPDATE ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'bookmark', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
END;

CREATE TRIGGER bookmark_sync_delete
AFTER DELETE ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'bookmark', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
END;

-- NOTES
CREATE TRIGGER notes_sync_insert
AFTER INSERT ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'notes', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
END;

CREATE TRIGGER notes_sync_update
AFTER UPDATE ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'notes', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
END;

CREATE TRIGGER notes_sync_delete
AFTER DELETE ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'notes', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
END;

-- RECYCLE_BIN
CREATE TRIGGER recycle_bin_sync_insert
AFTER INSERT ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'recycle_bin', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
END;

CREATE TRIGGER recycle_bin_sync_update
AFTER UPDATE ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'recycle_bin', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
END;

CREATE TRIGGER recycle_bin_sync_delete
AFTER DELETE ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'recycle_bin', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
END;

-- REMINDER
CREATE TRIGGER reminder_sync_insert
AFTER INSERT ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'reminder', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
END;

CREATE TRIGGER reminder_sync_update
AFTER UPDATE ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'reminder', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
END;

CREATE TRIGGER reminder_sync_delete
AFTER DELETE ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'reminder', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
END;

-- SNIPPETS
CREATE TRIGGER snippets_sync_insert
AFTER INSERT ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'snippets', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
END;

CREATE TRIGGER snippets_sync_update
AFTER UPDATE ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'snippets', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
END;

CREATE TRIGGER snippets_sync_delete
AFTER DELETE ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'snippets', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
END;

-- TODO
CREATE TRIGGER todo_sync_insert
AFTER INSERT ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'todo', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
END;

CREATE TRIGGER todo_sync_update
AFTER UPDATE ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'todo', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
END;

CREATE TRIGGER todo_sync_delete
AFTER DELETE ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'todo', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
END;

-- WORKSPACES
CREATE TRIGGER workspaces_sync_insert
AFTER INSERT ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'workspaces', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
END;

CREATE TRIGGER workspaces_sync_update
AFTER UPDATE ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'workspaces', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
END;

CREATE TRIGGER workspaces_sync_delete
AFTER DELETE ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'workspaces', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
END;

-- USER_PREFERENCE
CREATE TRIGGER user_preference_sync_insert
AFTER INSERT ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'user_preference', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
END;

CREATE TRIGGER user_preference_sync_update
AFTER UPDATE ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'user_preference', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
END;

CREATE TRIGGER user_preference_sync_delete
AFTER DELETE ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (randomblob(16), 'user_preference', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
END;
"#;
            db_connection.execute_unprepared(triggers).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();

        if db_backend == DbBackend::Sqlite {
            let drop_triggers = r#"
DROP TRIGGER IF EXISTS bookmark_sync_insert;
DROP TRIGGER IF EXISTS bookmark_sync_update;
DROP TRIGGER IF EXISTS bookmark_sync_delete;
DROP TRIGGER IF EXISTS notes_sync_insert;
DROP TRIGGER IF EXISTS notes_sync_update;
DROP TRIGGER IF EXISTS notes_sync_delete;
DROP TRIGGER IF EXISTS recycle_bin_sync_insert;
DROP TRIGGER IF EXISTS recycle_bin_sync_update;
DROP TRIGGER IF EXISTS recycle_bin_sync_delete;
DROP TRIGGER IF EXISTS reminder_sync_insert;
DROP TRIGGER IF EXISTS reminder_sync_update;
DROP TRIGGER IF EXISTS reminder_sync_delete;
DROP TRIGGER IF EXISTS snippets_sync_insert;
DROP TRIGGER IF EXISTS snippets_sync_update;
DROP TRIGGER IF EXISTS snippets_sync_delete;
DROP TRIGGER IF EXISTS todo_sync_insert;
DROP TRIGGER IF EXISTS todo_sync_update;
DROP TRIGGER IF EXISTS todo_sync_delete;
DROP TRIGGER IF EXISTS workspaces_sync_insert;
DROP TRIGGER IF EXISTS workspaces_sync_update;
DROP TRIGGER IF EXISTS workspaces_sync_delete;
DROP TRIGGER IF EXISTS user_preference_sync_insert;
DROP TRIGGER IF EXISTS user_preference_sync_update;
DROP TRIGGER IF EXISTS user_preference_sync_delete;
"#;
            db_connection.execute_unprepared(drop_triggers).await?;
        }

        Ok(())
    }
}
