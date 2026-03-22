BEGIN;

-- Drop & recreate FK
ALTER TABLE one_time_passwords
DROP CONSTRAINT IF EXISTS one_time_password_user_identifier_fkey;

ALTER TABLE one_time_passwords
ADD CONSTRAINT one_time_password_user_identifier_fkey
FOREIGN KEY (user_identifier)
REFERENCES users(identifier)
ON DELETE CASCADE;

COMMIT;
