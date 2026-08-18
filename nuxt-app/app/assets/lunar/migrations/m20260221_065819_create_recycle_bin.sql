-- ============================================
-- m20260221_065819_create_recycle_bin
-- ============================================

SELECT 1 FROM pg_type WHERE typname = 'item_type';

CREATE TYPE "item_type" AS ENUM ('todo', 'note', 'reminder', 'snippet', 'bookmark');

CREATE TABLE IF NOT EXISTS "recycle_bin" ( "identifier" uuid NOT NULL PRIMARY KEY, "item_id" uuid NOT NULL, "item_type" item_type NOT NULL, "payload" text NOT NULL, "deleted_at" timestamp with time zone NOT NULL );

