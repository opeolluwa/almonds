-- ============================================
-- m20260528_200000_add_workspace_and_is_read_to_notifications
-- ============================================

ALTER TABLE "notifications" ADD COLUMN IF NOT EXISTS "is_read" bool NOT NULL DEFAULT FALSE, ADD COLUMN IF NOT EXISTS "workspace_identifier" uuid;

ALTER TABLE "notifications" ADD CONSTRAINT "fk_notification_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

