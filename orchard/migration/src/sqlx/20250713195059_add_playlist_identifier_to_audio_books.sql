-- Add migration script here
ALTER TABLE audio_books
ADD COLUMN playlist_identifier UUID,
ADD CONSTRAINT fk_audio_books_playlist
    FOREIGN KEY (playlist_identifier)
    REFERENCES playlist(identifier)
    ON DELETE CASCADE
    ON UPDATE CASCADE;
