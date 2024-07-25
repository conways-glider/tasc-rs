-- Add up migration script here
CREATE TABLE users (
   id UUID PRIMARY KEY,
   username TEXT UNIQUE NOT NULL,
   name TEXT NOT NULL,
   passhash TEXT NULL,
   added TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_username
ON users(username);
