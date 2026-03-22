-- Add migration script here
CREATE TABLE
    one_time_password (
        identifier UUID PRIMARY KEY NOT NULL,
        user_identifier UUID NOT NULL REFERENCES users (identifier),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        otp CHAR(6) NOT NULL,
        updated_at TIMESTAMPTZ DEFAULT NULL
    )