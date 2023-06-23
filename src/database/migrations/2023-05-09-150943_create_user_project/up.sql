CREATE TABLE IF NOT EXISTS user_project ();

ALTER TABLE user_project
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id INTEGER NOT NULL,
  ADD COLUMN project_id INTEGER NOT NULL,
  ADD COLUMN active BOOLEAN NOT NULL DEFAULT TRUE,
  ADD COLUMN keys TEXT[] NOT NULL DEFAULT '{}',
  ADD COLUMN record JSONB DEFAULT '{}'
  ;

ALTER TABLE user_project
  ADD CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (id);

INSERT INTO user_project (user_id, project_id, keys) VALUES
  (1, 1, '{}')
  ;
