-- ============================================
-- m20260226_063044_make_notes_categories_optional
-- ============================================

ALTER TABLE "notes" ALTER COLUMN "categories" DROP NOT NULL;

