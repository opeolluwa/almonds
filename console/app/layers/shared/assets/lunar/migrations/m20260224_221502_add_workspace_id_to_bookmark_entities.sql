-- ============================================
-- m20260224_221502_add_workspace_id_to_bookmark_entities
-- ============================================

ALTER TABLE "bookmark" ADD COLUMN "workspace_identifier" uuid;

ALTER TABLE "bookmark" ADD CONSTRAINT "fk_bookmark_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

