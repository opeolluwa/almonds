-- ============================================
-- m20260224_221032_add_workspace_id_to_notes_entities
-- ============================================

ALTER TABLE "notes" ADD COLUMN "workspace_identifier" uuid;

ALTER TABLE "notes" ADD CONSTRAINT "fk_notes_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

