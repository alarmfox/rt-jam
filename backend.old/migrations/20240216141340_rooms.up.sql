-- Add up migration script here
CREATE TABLE IF NOT EXISTS rooms (
  id uuid PRIMARY KEY, 
  name VARCHAR(50) NOT NULL,
  description TEXT,
  owner VARCHAR(50) NOT NULL,
  private BOOLEAN NOT NULL DEFAULT FALSE,
  open BOOLEAN NOT NULL DEFAULT TRUE,
  max_people_playing INTEGER NOT NULL DEFAULT 5,

  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now(),

  CONSTRAINT fk_owner FOREIGN KEY(owner) REFERENCES users(username) ON DELETE CASCADE
)
