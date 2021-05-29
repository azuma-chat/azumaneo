ALTER TABLE textchannels
ADD COLUMN created_at timestamp with time zone NOT NULL DEFAULT current_timestamp
