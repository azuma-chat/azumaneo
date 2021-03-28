ALTER TABLE sessions
ADD COLUMN expires_at timestamp with time zone NOT NULL DEFAULT current_timestamp + (5 * interval '1 minute')
