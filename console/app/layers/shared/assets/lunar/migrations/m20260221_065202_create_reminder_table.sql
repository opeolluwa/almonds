-- ============================================
-- m20260221_065202_create_reminder_table
-- ============================================

CREATE TABLE IF NOT EXISTS "reminder" ( "identifier" uuid NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "description" text NULL, "recurring" bool NOT NULL DEFAULT FALSE, "recurrence_rule" varchar NULL, "alarm_sound" varchar NULL, "remind_at" timestamp with time zone NOT NULL, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );

