-- ============================================
-- m20260224_221622_add_workspace_id_to_reminder_entities
-- ============================================

ALTER TABLE "reminder" ADD COLUMN "workspace_identifier" uuid;

ALTER TABLE "reminder" ADD CONSTRAINT "fk_reminder_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

