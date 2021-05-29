ALTER TABLE messages
ADD FOREIGN KEY (author)
REFERENCES users(id),
ADD FOREIGN KEY (channel)
REFERENCES textchannels(id)
