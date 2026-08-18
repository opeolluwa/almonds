-- ============================================
-- m20260218_110353_create_note_category_table
-- ============================================

CREATE TABLE IF NOT EXISTS "note_categories" ( "identifier" uuid NOT NULL PRIMARY KEY, "label" varchar NOT NULL, "description" text NULL );

