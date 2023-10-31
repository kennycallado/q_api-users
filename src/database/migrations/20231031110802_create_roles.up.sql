CREATE TABLE IF NOT EXISTS roles ();

ALTER TABLE roles
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN name VARCHAR(10) NOT NULL;

INSERT INTO roles (name) VALUES
  ('admin'),
  ('coord'),
  ('thera'),
  ('user'),
  ('robot'),
  ('guest') ON CONFLICT DO NOTHING
  ;
