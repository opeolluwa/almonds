-- ============================================
-- m20260218_204212_create_bookmarks_table
-- ============================================

SELECT 1 FROM pg_type WHERE typname = 'tag';

CREATE TYPE "tag" AS ENUM ('development', 'inspiration', 'design', 'research');

CREATE TABLE IF NOT EXISTS "bookmark" ( "identifier" uuid NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "url" varchar NOT NULL, "tag" tag NOT NULL DEFAULT 'development', "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );

