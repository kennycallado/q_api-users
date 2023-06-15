CREATE TABLE IF NOT EXISTS user_project ();

ALTER TABLE user_project
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id INTEGER NOT NULL,
  ADD COLUMN project_id INTEGER NOT NULL,
  ADD COLUMN keys TEXT[] NOT NULL DEFAULT '{}',
  ADD COLUMN record JSONB NOT NULL DEFAULT '{}'
  ;

INSERT INTO user_project (user_id, project_id, keys) VALUES
  (1, 1, '{}')
  ;
