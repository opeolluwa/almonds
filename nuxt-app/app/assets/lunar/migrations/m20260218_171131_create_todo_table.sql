-- ============================================
-- m20260218_171131_create_todo_table
-- ============================================

SELECT 1 FROM pg_type WHERE typname = 'priority';

CREATE TYPE "priority" AS ENUM ('high', 'medium', 'low');

CREATE TABLE IF NOT EXISTS "todo" ( "identifier" uuid NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "description" text NULL, "due_date" date NULL, "priority" priority NOT NULL DEFAULT 'medium', "done" bool NOT NULL DEFAULT FALSE, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );

