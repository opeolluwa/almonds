-- ============================================
-- m20260527_110634_create_user_preference_table
-- ============================================

CREATE TABLE IF NOT EXISTS "user_preferences" ( "identifier" uuid NOT NULL PRIMARY KEY, "master_first_name" varchar NOT NULL, "master_last_name" varchar NOT NULL, "master_email" varchar NOT NULL, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );

