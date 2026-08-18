-- ============================================
-- m20260218_110352_create_note_table
-- ============================================

CREATE TABLE IF NOT EXISTS "notes" ( "identifier" uuid NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "content" text NOT NULL, "categories" json NOT NULL, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );

