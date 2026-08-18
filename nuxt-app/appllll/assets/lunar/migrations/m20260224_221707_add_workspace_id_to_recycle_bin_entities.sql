-- ============================================
-- m20260224_221707_add_workspace_id_to_recycle_bin_entities
-- ============================================

ALTER TABLE "recycle_bin" ADD COLUMN "workspace_identifier" uuid;

ALTER TABLE "recycle_bin" ADD CONSTRAINT "fk_recycle_bin_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

