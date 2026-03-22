-- Add migration script here
CREATE TABLE
    IF NOT EXISTS audio_books (
        identifier UUID PRIMARY KEY,
        user_identifier UUID NOT NULL REFERENCES users (identifier),
        src VARCHAR NOT NULL,
        name VARCHAR NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ DEFAULT NULL,
        last_played TIMESTAMPTZ DEFAULT NULL
    )