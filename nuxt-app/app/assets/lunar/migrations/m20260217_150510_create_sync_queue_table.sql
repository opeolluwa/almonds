-- ============================================
-- m20260217_150510_create_sync_queue_table
-- ============================================

CREATE TABLE IF NOT EXISTS "sync_queue" ( "identifier" uuid NOT NULL PRIMARY KEY, "table_name" varchar NOT NULL, "record_identifier" varchar NOT NULL, "operation" varchar NOT NULL, "created_at" varchar NOT NULL );

