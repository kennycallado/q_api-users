CREATE TABLE IF NOT EXISTS users ();

ALTER TABLE users
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN depends_on INTEGER NOT NULL,
  ADD COLUMN role_id INTEGER NOT NULL DEFAULT 4,
  ADD COLUMN user_token VARCHAR(60),
  ADD COLUMN active BOOLEAN NOT NULL DEFAULT TRUE,
  ADD COLUMN created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  ADD CONSTRAINT fk_users_depends_on FOREIGN KEY (depends_on) REFERENCES users(id) ON DELETE CASCADE,
  ADD CONSTRAINT fk_users_role FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
  ;

SELECT diesel_manage_updated_at('users');
INSERT INTO users (user_token, depends_on, role_id) VALUES
  ('admin_user',  1, 1),
  ('coord1_user', 1, 2),
  ('coord2_user', 1, 2),
  ('thera1_user', 2, 3),
  ('thera2_user', 2, 3),
  ('thera3_user', 3, 3),
  ('user1_user',  3, default),
  ('user2_user',  4, default),
  ('user3_user',  5, default),
  ('user4_user',  3, default),
  ('user5_user',  4, 4)
  ;

ALTER TABLE users REPLICA IDENTITY FULL;
CREATE PUBLICATION users_pub FOR TABLE users;
