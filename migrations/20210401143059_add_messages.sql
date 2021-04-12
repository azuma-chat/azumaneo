CREATE TABLE messages (
    id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    authorid uuid NOT NULL,
    channelid uuid NOT NULL,
    content text,
    "timestamp" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
)