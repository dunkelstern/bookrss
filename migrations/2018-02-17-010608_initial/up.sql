CREATE TABLE author (
  id INTEGER PRIMARY KEY NOT NULL,
  language VARCHAR NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE speaker (
  id INTEGER PRIMARY KEY NOT NULL,
  language VARCHAR NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE series (
  id INTEGER PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  translation VARCHAR NOT NULL,
  description TEXT,
  author_id INTEGER NOT NULL,
  FOREIGN KEY(author_id) REFERENCES author(id)
);

CREATE TABLE audiobook (
  id INTEGER PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  description TEXT,
  part_no INTEGER NOT NULL,
  publish_date TEXT,
  speaker_id INTEGER NOT NULL,
  series_id INTEGER NOT NULL,
  FOREIGN KEY(speaker_id) REFERENCES speaker(id),
  FOREIGN KEY(series_id) REFERENCES series(id)
);

CREATE TABLE part (
  id INTEGER PRIMARY KEY NOT NULL,
  file_name VARCHAR NOT NULL,
  file_size INTEGER NOT NULL,
  start_time INTEGER NOT NULL,
  duration INTEGER NOT NULL,
  audiobook_id INTEGER NOT NULL,
  FOREIGN KEY(audiobook_id) REFERENCES audiobook(id)
);
