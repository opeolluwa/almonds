-- ============================================
-- m20260528_132342_notification
-- ============================================

SELECT 1 FROM pg_type WHERE typname = 'notification_type';

CREATE TYPE "notification_type" AS ENUM ('backup_failed', 'backup_success', 'workspace_invite_received', 'workspace_invite_accepted', 'workspace_invite_declined', 'item_shared', 'item_unshared', 'item_updated', 'item_deleted', 'item_access_granted', 'item_access_revoked', 'generic');

CREATE TABLE IF NOT EXISTS "notifications" ( "identifier" uuid NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "body" varchar NOT NULL, "notification_type" notification_type NOT NULL DEFAULT 'generic', "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );

