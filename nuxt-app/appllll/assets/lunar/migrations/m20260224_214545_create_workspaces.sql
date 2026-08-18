-- ============================================
-- m20260224_214545_create_workspaces
-- ============================================

CREATE TABLE IF NOT EXISTS "workspaces" ( "name" varchar NOT NULL, "description" varchar NOT NULL, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL, "identifier" uuid NOT NULL PRIMARY KEY );

