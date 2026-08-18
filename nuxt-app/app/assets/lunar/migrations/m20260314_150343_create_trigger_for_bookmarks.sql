-- ============================================
-- m20260314_150343_create_trigger_for_bookmarks
-- ============================================



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

;


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
;

