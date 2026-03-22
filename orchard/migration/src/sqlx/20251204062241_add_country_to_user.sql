-- Add migration script here

ALTER TABLE users ADD COLUMN country_identifier CHAR(26) REFERENCES countries(identifier)