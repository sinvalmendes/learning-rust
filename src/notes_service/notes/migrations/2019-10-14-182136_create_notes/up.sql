-- Your SQL goes here
CREATE TABLE notes (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  content VARCHAR NOT NULL
)