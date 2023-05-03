CREATE TABLE IF NOT EXISTS roles ();
-- CREATE PUBLICATION roles_pub FOR TABLE roles;

ALTER TABLE roles
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN name VARCHAR(10) NOT NULL;

-- ALTER TABLE roles REPLICA IDENTITY FULL;

INSERT INTO roles (name) VALUES
  ('admin'),
  ('coord'),
  ('thera'),
  ('user'),
  ('robot'),
  ('guest') ON CONFLICT DO NOTHING
  ;
