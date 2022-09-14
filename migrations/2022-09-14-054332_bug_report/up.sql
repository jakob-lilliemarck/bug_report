CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  name VARCHAR(32) NOT NULL
);

CREATE TABLE script_tag_assoc (
  id SERIAL PRIMARY KEY,
  tag_id INTEGER NOT NULL,
  is_output BOOLEAN NOT NULL DEFAULT false,
  FOREIGN KEY (tag_id) REFERENCES tags (id)
);
