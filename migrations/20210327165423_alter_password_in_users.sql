DELETE FROM sessions; -- delete all users and their sessions, as the hashing algorithm was changed
DELETE FROM users; -- this shouldn't be a problem at the current stage of development
ALTER TABLE users
ALTER COLUMN password TYPE bytea USING password::bytea
