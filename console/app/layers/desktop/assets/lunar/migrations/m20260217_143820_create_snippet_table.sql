-- ============================================
-- m20260217_143820_create_snippet_table
-- ============================================

CREATE TABLE IF NOT EXISTS "snippets" ( "identifier" uuid NOT NULL PRIMARY KEY, "title" varchar NULL, "language" varchar NULL, "code" text NOT NULL, "description" text NULL, "is_pinned" bool NOT NULL DEFAULT FALSE, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );

