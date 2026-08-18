-- ============================================
-- m20260518_000000_remove_email_unique_constraint_from_user_preference
-- ============================================

ALTER TABLE user_preference DROP CONSTRAINT user_preference_email_key;

