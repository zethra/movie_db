CREATE TABLE IF NOT EXISTS movies (
  movies_id VARCHAR PRIMARY KEY NOT NULL,
  movies_name VARCHAR NOT NULL,
  movies_rating VARCHAR NOT NULL,
  movies_category VARCHAR NOT NULL,
  movies_format VARCHAR NOT NULL,
  movies_aspect VARCHAR NOT NULL
);