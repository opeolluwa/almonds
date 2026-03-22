use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();

        if db_backend == DbBackend::Sqlite {
            let triggers = r#"

-- BOOKMARK
CREATE TRIGGER bookmark_sync_insert
AFTER INSERT ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'bookmark', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER bookmark_sync_update
AFTER UPDATE ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'bookmark', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER bookmark_sync_delete
AFTER DELETE ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'bookmark', OLD.identifier, 'DELETE', datetime('now'));
END;


-- NOTES
CREATE TRIGGER notes_sync_insert
AFTER INSERT ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'notes', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER notes_sync_update
AFTER UPDATE ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'notes', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER notes_sync_delete
AFTER DELETE ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'notes', OLD.identifier, 'DELETE', datetime('now'));
END;


-- RECYCLE_BIN
CREATE TRIGGER recycle_bin_sync_insert
AFTER INSERT ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'recycle_bin', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER recycle_bin_sync_update
AFTER UPDATE ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'recycle_bin', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER recycle_bin_sync_delete
AFTER DELETE ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'recycle_bin', OLD.identifier, 'DELETE', datetime('now'));
END;


-- REMINDER
CREATE TRIGGER reminder_sync_insert
AFTER INSERT ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'reminder', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER reminder_sync_update
AFTER UPDATE ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'reminder', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER reminder_sync_delete
AFTER DELETE ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'reminder', OLD.identifier, 'DELETE', datetime('now'));
END;


-- SNIPPETS
CREATE TRIGGER snippets_sync_insert
AFTER INSERT ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'snippets', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER snippets_sync_update
AFTER UPDATE ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'snippets', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER snippets_sync_delete
AFTER DELETE ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'snippets', OLD.identifier, 'DELETE', datetime('now'));
END;


-- TODO
CREATE TRIGGER todo_sync_insert
AFTER INSERT ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'todo', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER todo_sync_update
AFTER UPDATE ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'todo', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER todo_sync_delete
AFTER DELETE ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'todo', OLD.identifier, 'DELETE', datetime('now'));
END;


-- WORKSPACES
CREATE TRIGGER workspaces_sync_insert
AFTER INSERT ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'workspaces', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER workspaces_sync_update
AFTER UPDATE ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'workspaces', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER workspaces_sync_delete
AFTER DELETE ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'workspaces', OLD.identifier, 'DELETE', datetime('now'));
END;


-- USER_PREFERENCE
CREATE TRIGGER user_preference_sync_insert
AFTER INSERT ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'user_preference', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER user_preference_sync_update
AFTER UPDATE ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'user_preference', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER user_preference_sync_delete
AFTER DELETE ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'user_preference', OLD.identifier, 'DELETE', datetime('now'));
END;

"#;

            db_connection.execute_unprepared(triggers).await?;
        }

        if db_backend == DbBackend::Postgres {
            let trigger_function = r#"

CREATE OR REPLACE FUNCTION enqueue_sync()
RETURNS TRIGGER AS $$
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (
gen_random_uuid(),
TG_TABLE_NAME,
COALESCE(NEW.identifier, OLD.identifier)::text,
TG_OP,
NOW()::text
);

RETURN NEW;
END;
$$ LANGUAGE plpgsql;

"#;

            db_connection.execute_unprepared(trigger_function).await?;

            let triggers = r#"
-- TODO
CREATE TRIGGER todo_sync_trigger
AFTER INSERT OR UPDATE OR DELETE ON todo
FOR EACH ROW EXECUTE FUNCTION enqueue_sync();

-- BOOKMARK
CREATE TRIGGER bookmarks_sync_trigger
AFTER INSERT OR UPDATE OR DELETE ON bookmark
FOR EACH ROW EXECUTE FUNCTION enqueue_sync();

-- NOTES
CREATE TRIGGER notes_sync_trigger
AFTER INSERT OR UPDATE OR DELETE ON notes
FOR EACH ROW EXECUTE FUNCTION enqueue_sync();

-- RECYCLE BIN
CREATE TRIGGER recycle_bin_sync_trigger
AFTER INSERT OR UPDATE OR DELETE ON recycle_bin
FOR EACH ROW EXECUTE FUNCTION enqueue_sync();

-- USER PREFERENCE
CREATE TRIGGER user_preference_sync_trigger
AFTER INSERT OR UPDATE OR DELETE ON user_preference
FOR EACH ROW EXECUTE FUNCTION enqueue_sync();

-- REMINDER
CREATE TRIGGER reminder_sync_trigger
AFTER INSERT OR UPDATE OR DELETE ON reminder
FOR EACH ROW EXECUTE FUNCTION enqueue_sync();

-- SNIPPETS (fixed table name)
CREATE TRIGGER snippets_sync_trigger
AFTER INSERT OR UPDATE OR DELETE ON snippets
FOR EACH ROW EXECUTE FUNCTION enqueue_sync();

-- WORKSPACES
CREATE TRIGGER workspaces_sync_trigger
AFTER INSERT OR UPDATE OR DELETE ON workspaces
FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
"#;
            db_connection.execute_unprepared(triggers).await?;
        }

        if db_backend == DbBackend::MySql {
            let triggers = r#"

CREATE TRIGGER bookmark_sync_insert
AFTER INSERT ON bookmark
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'bookmark', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER bookmark_sync_update
AFTER UPDATE ON bookmark
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'bookmark', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER bookmark_sync_delete
AFTER DELETE ON bookmark
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'bookmark', OLD.identifier, 'DELETE', NOW());


-- NOTES
CREATE TRIGGER notes_sync_insert
AFTER INSERT ON notes
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'notes', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER notes_sync_update
AFTER UPDATE ON notes
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'notes', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER notes_sync_delete
AFTER DELETE ON notes
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'notes', OLD.identifier, 'DELETE', NOW());


-- RECYCLE_BIN
CREATE TRIGGER recycle_bin_sync_insert
AFTER INSERT ON recycle_bin
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'recycle_bin', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER recycle_bin_sync_update
AFTER UPDATE ON recycle_bin
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'recycle_bin', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER recycle_bin_sync_delete
AFTER DELETE ON recycle_bin
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'recycle_bin', OLD.identifier, 'DELETE', NOW());


-- REMINDER
CREATE TRIGGER reminder_sync_insert
AFTER INSERT ON reminder
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'reminder', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER reminder_sync_update
AFTER UPDATE ON reminder
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'reminder', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER reminder_sync_delete
AFTER DELETE ON reminder
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'reminder', OLD.identifier, 'DELETE', NOW());


-- SNIPPETS
CREATE TRIGGER snippets_sync_insert
AFTER INSERT ON snippets
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'snippets', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER snippets_sync_update
AFTER UPDATE ON snippets
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'snippets', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER snippets_sync_delete
AFTER DELETE ON snippets
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'snippets', OLD.identifier, 'DELETE', NOW());


CREATE TRIGGER todo_sync_insert
AFTER INSERT ON todo
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'todo', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER todo_sync_update
AFTER UPDATE ON todo
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'todo', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER todo_sync_delete
AFTER DELETE ON todo
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'todo', OLD.identifier, 'DELETE', NOW());

-- WORKSPACES
CREATE TRIGGER workspaces_sync_insert
AFTER INSERT ON workspaces
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'workspaces', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER workspaces_sync_update
AFTER UPDATE ON workspaces
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'workspaces', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER workspaces_sync_delete
AFTER DELETE ON workspaces
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'workspaces', OLD.identifier, 'DELETE', NOW());


-- USER_PREFERENCE
CREATE TRIGGER user_preference_sync_insert
AFTER INSERT ON user_preference
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'user_preference', NEW.identifier, 'INSERT', NOW());

CREATE TRIGGER user_preference_sync_update
AFTER UPDATE ON user_preference
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'user_preference', NEW.identifier, 'UPDATE', NOW());

CREATE TRIGGER user_preference_sync_delete
AFTER DELETE ON user_preference
FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (UUID(), 'user_preference', OLD.identifier, 'DELETE', NOW());

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

        if db_backend == DbBackend::Postgres {
            let drop_triggers = r#"
DROP TRIGGER IF EXISTS todo_sync_trigger ON todo;
DROP TRIGGER IF EXISTS bookmarks_sync_trigger ON bookmark;
DROP TRIGGER IF EXISTS notes_sync_trigger ON notes;
DROP TRIGGER IF EXISTS recycle_bin_sync_trigger ON recycle_bin;
DROP TRIGGER IF EXISTS user_preference_sync_trigger ON user_preference;
DROP TRIGGER IF EXISTS reminder_sync_trigger ON reminder;
DROP TRIGGER IF EXISTS snippets_sync_trigger ON snippets;
DROP TRIGGER IF EXISTS workspaces_sync_trigger ON workspaces;

DROP FUNCTION IF EXISTS enqueue_sync;
"#;

            db_connection.execute_unprepared(drop_triggers).await?;
        }

        if db_backend == DbBackend::MySql {
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
