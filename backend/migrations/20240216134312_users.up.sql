-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY, 
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(255) UNIQUE NOT NULL,
  password TEXT DEFAULT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now(),
  verification_token VARCHAR(255) DEFAULT NULL,
  verification_token_expires_in TIMESTAMPTZ DEFAULT NULL,
  enabled BOOLEAN NOT NULL DEFAULT TRUE
);
