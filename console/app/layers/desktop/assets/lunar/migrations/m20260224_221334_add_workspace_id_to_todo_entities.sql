-- ============================================
-- m20260224_221334_add_workspace_id_to_todo_entities
-- ============================================

ALTER TABLE "todo" ADD COLUMN "workspace_identifier" uuid;

ALTER TABLE "todo" ADD CONSTRAINT "fk_todo_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

