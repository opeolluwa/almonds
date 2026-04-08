-- Add migration script here
CREATE TABLE
    IF NOT EXISTS notifications (
        identifier UUID NOT NULL PRIMARY KEY,
        subject VARCHAR NOT NULL,
        body VARCHAR NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ DEFAULT NULL,
        is_read BOOLEAN DEFAULT FALSE,
        user_identifier UUID REFERENCES users (identifier) ON UPDATE CASCADE ON DELETE CASCADE
    )