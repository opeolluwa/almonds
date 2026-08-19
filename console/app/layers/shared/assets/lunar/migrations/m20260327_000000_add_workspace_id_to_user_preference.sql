-- ============================================
-- m20260327_000000_add_workspace_id_to_user_preference
-- ============================================

ALTER TABLE "user_preference" ADD COLUMN "workspace_identifier" uuid;

ALTER TABLE "user_preference" ADD CONSTRAINT "fk_user_preference_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

