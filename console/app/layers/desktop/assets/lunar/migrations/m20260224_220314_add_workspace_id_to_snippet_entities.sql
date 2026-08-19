-- ============================================
-- m20260224_220314_add_workspace_id_to_snippet_entities
-- ============================================

ALTER TABLE "snippets" ADD COLUMN "workspace_identifier" uuid;

ALTER TABLE "snippets" ADD CONSTRAINT "fk_snippets_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

