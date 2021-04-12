CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL UNIQUE,
    password text NOT NULL,
    created timestamp with time zone NOT NULL DEFAULT current_timestamp
)