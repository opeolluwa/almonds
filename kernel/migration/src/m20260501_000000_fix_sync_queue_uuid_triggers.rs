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

        if db_backend == DbBackend::MySql {
            // MySQL triggers created by m20260314 used UUID() which returns a 36-char string,
            // but sync_queue.identifier is BINARY(16). Drop and recreate with UUID_TO_BIN(UUID()).
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
CREATE TRIGGER bookmark_sync_insert
AFTER INSERT ON bookmark
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'bookmark', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER bookmark_sync_update
AFTER UPDATE ON bookmark
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'bookmark', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER bookmark_sync_delete
AFTER DELETE ON bookmark
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'bookmark', OLD.identifier, 'DELETE', NOW());

CREATE TRIGGER notes_sync_insert
AFTER INSERT ON notes
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'notes', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER notes_sync_update
AFTER UPDATE ON notes
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'notes', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER notes_sync_delete
AFTER DELETE ON notes
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'notes', OLD.identifier, 'DELETE', NOW());

CREATE TRIGGER recycle_bin_sync_insert
AFTER INSERT ON recycle_bin
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'recycle_bin', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER recycle_bin_sync_update
AFTER UPDATE ON recycle_bin
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'recycle_bin', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER recycle_bin_sync_delete
AFTER DELETE ON recycle_bin
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'recycle_bin', OLD.identifier, 'DELETE', NOW());

CREATE TRIGGER reminder_sync_insert
AFTER INSERT ON reminder
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'reminder', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER reminder_sync_update
AFTER UPDATE ON reminder
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'reminder', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER reminder_sync_delete
AFTER DELETE ON reminder
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'reminder', OLD.identifier, 'DELETE', NOW());

CREATE TRIGGER snippets_sync_insert
AFTER INSERT ON snippets
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'snippets', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER snippets_sync_update
AFTER UPDATE ON snippets
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'snippets', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER snippets_sync_delete
AFTER DELETE ON snippets
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'snippets', OLD.identifier, 'DELETE', NOW());

CREATE TRIGGER todo_sync_insert
AFTER INSERT ON todo
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'todo', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER todo_sync_update
AFTER UPDATE ON todo
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'todo', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER todo_sync_delete
AFTER DELETE ON todo
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'todo', OLD.identifier, 'DELETE', NOW());

CREATE TRIGGER workspaces_sync_insert
AFTER INSERT ON workspaces
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'workspaces', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER workspaces_sync_update
AFTER UPDATE ON workspaces
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'workspaces', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER workspaces_sync_delete
AFTER DELETE ON workspaces
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'workspaces', OLD.identifier, 'DELETE', NOW());

CREATE TRIGGER user_preference_sync_insert
AFTER INSERT ON user_preference
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'user_preference', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER user_preference_sync_update
AFTER UPDATE ON user_preference
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'user_preference', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER user_preference_sync_delete
AFTER DELETE ON user_preference
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID_TO_BIN(UUID()), 'user_preference', OLD.identifier, 'DELETE', NOW());
"#;
            db_connection.execute_unprepared(triggers).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();

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
        if db_backend == DbBackend::Sqlite || db_backend == DbBackend::MySql {
            db_connection.execute_unprepared(drop_triggers).await?;
        }

        Ok(())
    }
}
