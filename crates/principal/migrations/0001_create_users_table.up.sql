-- Add up migration script here
CREATE TABLE users (
   id UUID PRIMARY KEY,
   name TEXT NOT NULL,
   passhash TEXT NULL,
   added TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
