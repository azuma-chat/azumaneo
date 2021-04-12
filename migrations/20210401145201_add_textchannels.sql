CREATE TABLE textchannels (
    id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    name text NOT NULL,
    description text
)