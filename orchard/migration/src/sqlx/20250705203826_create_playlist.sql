-- Add migration script here
CREATE TABLE
    IF NOT EXISTS playlist (
        identifier UUID NOT NULL PRIMARY KEY,
        name VARCHAR NOT NULL,
        about VARCHAR,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ DEFAULT NULL
    )