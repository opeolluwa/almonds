-- Add migration script here
ALTER TABLE one_time_password
RENAME COLUMN otp TO code;