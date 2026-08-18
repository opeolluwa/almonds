-- ============================================
-- m20260527_171026_remove_email_from_workspace_preference
-- ============================================

ALTER TABLE "workspace_preferences" DROP COLUMN IF EXISTS "email";

