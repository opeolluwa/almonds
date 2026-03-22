-- Add migration script here
ALTER TABLE playlist
    ADD COLUMN user_identifier UUID NOT NULL, ADD CONSTRAINT fk_playlist_users
    FOREIGN KEY (user_identifier) REFERENCES users(identifier) ON
UPDATE CASCADE;