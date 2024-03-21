-- Add up migration script here
CREATE TABLE IF NOT EXISTS sessions (
  id TEXT PRIMARY KEY,
  data BYTEA NOT NULL,
  expiry_date TIMESTAMPTZ NOT NULL
);
