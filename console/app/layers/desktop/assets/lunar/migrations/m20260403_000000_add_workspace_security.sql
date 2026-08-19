-- ============================================
-- m20260403_000000_add_workspace_security
-- ============================================


                ALTER TABLE workspaces ADD COLUMN IF NOT EXISTS is_secured BOOLEAN NOT NULL DEFAULT FALSE;
                ALTER TABLE workspaces ADD COLUMN IF NOT EXISTS password_hash TEXT;
                ;

