-- ============================================
-- m20260331_000000_add_workspace_flags
-- ============================================


                ALTER TABLE workspaces ADD COLUMN IF NOT EXISTS is_default BOOLEAN NOT NULL DEFAULT FALSE;
                ALTER TABLE workspaces ADD COLUMN IF NOT EXISTS is_hidden BOOLEAN NOT NULL DEFAULT FALSE;
                UPDATE workspaces SET is_default = TRUE WHERE name = 'default';
                ;

